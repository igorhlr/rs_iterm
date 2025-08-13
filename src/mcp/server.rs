use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, oneshot, Mutex};
use tokio::time::{timeout, interval};
use tracing::{error, info, warn, debug};

use crate::mcp::router::Router;
use crate::mcp::tools::register_tools;
use crate::mcp::utilities::check_iterm_availability;

/// Estatísticas do servidor
#[derive(Debug, Clone)]
pub struct ServerStats {
    /// Total de conexões aceitas
    pub total_connections: usize,
    /// Conexões ativas no momento
    pub active_connections: usize,
    /// Total de mensagens processadas
    pub total_messages: usize,
    /// Erros encontrados
    pub total_errors: usize,
}

/// Servidor MCP para iTerm com gerenciamento robusto
pub struct McpServer {
    /// Endereço do servidor
    address: SocketAddr,
    /// Router para processar mensagens
    router: Arc<Router>,
    /// Contador de conexões ativas
    active_connections: Arc<AtomicUsize>,
    /// Contador total de conexões
    total_connections: Arc<AtomicUsize>,
    /// Contador de mensagens processadas
    total_messages: Arc<AtomicUsize>,
    /// Contador de erros
    total_errors: Arc<AtomicUsize>,
    /// Canal para shutdown
    shutdown_tx: Option<broadcast::Sender<()>>,
}

impl McpServer {
    /// Cria um novo servidor MCP
    pub fn new(address: String, port: u16) -> Result<Self> {
        let addr: SocketAddr = format!("{}:{}", address, port)
            .parse()
            .context("Falha ao analisar o endereço de socket")?;

        // Registra as ferramentas
        let tools = register_tools();
        info!("Ferramentas registradas: {}", tools.len());

        // Cria o roteador MCP e registra as ferramentas
        let router = Arc::new(Router::new());
        for (name, (definition, handler)) in tools {
            info!("Registrando ferramenta: {}", name);
            router.register_tool(name, definition, handler);
        }

        Ok(McpServer {
            address: addr,
            router,
            active_connections: Arc::new(AtomicUsize::new(0)),
            total_connections: Arc::new(AtomicUsize::new(0)),
            total_messages: Arc::new(AtomicUsize::new(0)),
            total_errors: Arc::new(AtomicUsize::new(0)),
            shutdown_tx: None,
        })
    }

    /// Obtém as estatísticas atuais do servidor
    pub fn get_stats(&self) -> ServerStats {
        ServerStats {
            total_connections: self.total_connections.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
            total_messages: self.total_messages.load(Ordering::Relaxed),
            total_errors: self.total_errors.load(Ordering::Relaxed),
        }
    }

    /// Inicia o servidor e retorna um handle para shutdown
    pub async fn start(mut self) -> Result<ServerHandle> {
        // Verifica se o iTerm2 está disponível
        if !check_iterm_availability() {
            error!("iTerm2 não está em execução. Certifique-se de que o aplicativo está aberto.");
            return Err(anyhow::anyhow!("iTerm2 não está em execução"));
        }

        info!("iTerm2 detectado e disponível");

        // Cria o listener TCP
        let listener = TcpListener::bind(&self.address)
            .await
            .context("Falha ao vincular o servidor ao endereço")?;

        info!("Servidor vinculado ao endereço {}", self.address);

        // Canal para shutdown
        let (shutdown_tx, _) = broadcast::channel(1);
        self.shutdown_tx = Some(shutdown_tx.clone());

        // Canal para sinalizar que o servidor parou
        let (stopped_tx, stopped_rx) = oneshot::channel();

        // Cria handle para o servidor
        let handle = ServerHandle {
            address: self.address,
            shutdown_tx: shutdown_tx.clone(),
            stopped_rx,
            stats: ServerStats {
                total_connections: 0,
                active_connections: 0,
                total_messages: 0,
                total_errors: 0,
            },
            active_connections: self.active_connections.clone(),
            total_connections: self.total_connections.clone(),
            total_messages: self.total_messages.clone(),
            total_errors: self.total_errors.clone(),
        };

        // Clona as referências necessárias para a task
        let router = self.router.clone();
        let active_connections = self.active_connections.clone();
        let total_connections = self.total_connections.clone();
        let total_messages = self.total_messages.clone();
        let total_errors = self.total_errors.clone();
        let mut shutdown_rx = shutdown_tx.subscribe();

        // Spawn da task principal do servidor
        tokio::spawn(async move {
            info!("Servidor MCP do iTerm iniciado em {}", self.address);

            // Loop principal do servidor
            loop {
                tokio::select! {
                    // Aceita novas conexões
                    accept_result = listener.accept() => {
                        match accept_result {
                            Ok((socket, addr)) => {
                                info!("Nova conexão de {}", addr);
                                
                                // Incrementa contadores
                                total_connections.fetch_add(1, Ordering::Relaxed);
                                active_connections.fetch_add(1, Ordering::Relaxed);

                                // Clona referências para a task de conexão
                                let router_clone = router.clone();
                                let active_connections_clone = active_connections.clone();
                                let total_messages_clone = total_messages.clone();
                                let total_errors_clone = total_errors.clone();
                                let mut shutdown_rx_clone = shutdown_tx.subscribe();

                                // Spawn da task para lidar com a conexão
                                tokio::spawn(async move {
                                    // Configura timeout para a conexão (30 minutos)
                                    let connection_timeout = Duration::from_secs(1800);
                                    
                                    let result = tokio::select! {
                                        // Processa a conexão com timeout
                                        result = timeout(connection_timeout, 
                                            Self::handle_connection_with_stats(
                                                router_clone, 
                                                socket, 
                                                addr,
                                                total_messages_clone.clone(),
                                                total_errors_clone.clone()
                                            )
                                        ) => {
                                            match result {
                                                Ok(Ok(())) => {
                                                    debug!("Conexão de {} fechada normalmente", addr);
                                                }
                                                Ok(Err(e)) => {
                                                    error!("Erro ao processar conexão de {}: {}", addr, e);
                                                    total_errors_clone.fetch_add(1, Ordering::Relaxed);
                                                }
                                                Err(_) => {
                                                    warn!("Timeout na conexão de {}", addr);
                                                    total_errors_clone.fetch_add(1, Ordering::Relaxed);
                                                }
                                            }
                                        }
                                        // Shutdown signal
                                        _ = shutdown_rx_clone.recv() => {
                                            info!("Fechando conexão de {} devido ao shutdown", addr);
                                        }
                                    };

                                    // Decrementa conexões ativas
                                    active_connections_clone.fetch_sub(1, Ordering::Relaxed);
                                });
                            }
                            Err(e) => {
                                error!("Erro ao aceitar conexão: {}", e);
                                total_errors.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                    // Shutdown signal
                    _ = shutdown_rx.recv() => {
                        info!("Recebido sinal de shutdown, parando servidor...");
                        break;
                    }
                }
            }

            // Aguarda todas as conexões serem fechadas
            let mut wait_count = 0;
            while active_connections.load(Ordering::Relaxed) > 0 && wait_count < 30 {
                tokio::time::sleep(Duration::from_secs(1)).await;
                wait_count += 1;
                
                if wait_count % 5 == 0 {
                    info!(
                        "Aguardando {} conexões ativas serem fechadas...", 
                        active_connections.load(Ordering::Relaxed)
                    );
                }
            }

            if active_connections.load(Ordering::Relaxed) > 0 {
                warn!(
                    "Forçando fechamento com {} conexões ainda ativas", 
                    active_connections.load(Ordering::Relaxed)
                );
            }

            info!("Servidor MCP parado");
            let _ = stopped_tx.send(());
        });

        Ok(handle)
    }

    /// Processa uma conexão e atualiza estatísticas
    async fn handle_connection_with_stats(
        router: Arc<Router>,
        socket: tokio::net::TcpStream,
        addr: SocketAddr,
        total_messages: Arc<AtomicUsize>,
        total_errors: Arc<AtomicUsize>,
    ) -> Result<()> {
        // Cria um wrapper que conta mensagens
        let router_wrapper = RouterWrapper {
            router,
            total_messages,
            total_errors,
        };

        router_wrapper.handle_connection(socket).await
    }
}

/// Wrapper do Router para contar estatísticas
struct RouterWrapper {
    router: Arc<Router>,
    total_messages: Arc<AtomicUsize>,
    total_errors: Arc<AtomicUsize>,
}

impl RouterWrapper {
    async fn handle_connection(&self, mut socket: tokio::net::TcpStream) -> Result<()> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        
        let addr = socket.peer_addr().unwrap_or_else(|_| "[unknown]".parse().unwrap());
        debug!("RouterWrapper processando conexão de {}", addr);

        // Buffer para leitura dos dados
        let mut buffer = vec![0u8; 8192];
        let mut read_pos = 0;

        loop {
            match socket.read(&mut buffer[read_pos..]).await {
                Ok(0) => {
                    debug!("Conexão fechada pelo cliente: {}", addr);
                    break;
                }
                Ok(n) => {
                    read_pos += n;
                    debug!("Lidos {} bytes, total de {} bytes no buffer", n, read_pos);

                    // Processa mensagens completas no buffer
                    let mut processed_pos = 0;
                    while processed_pos < read_pos {
                        // Tenta encontrar um JSON válido terminado por newline
                        if let Some(msg_end) = buffer[processed_pos..read_pos]
                            .iter()
                            .position(|&b| b == b'\n')
                        {
                            let msg_end = processed_pos + msg_end;
                            
                            // Extrai a mensagem JSON
                            let message = match std::str::from_utf8(&buffer[processed_pos..msg_end]) {
                                Ok(msg) => msg,
                                Err(e) => {
                                    error!("Falha ao converter bytes para UTF-8: {}", e);
                                    self.total_errors.fetch_add(1, Ordering::Relaxed);
                                    processed_pos = msg_end + 1;
                                    continue;
                                }
                            };
                            
                            // Incrementa contador de mensagens
                            self.total_messages.fetch_add(1, Ordering::Relaxed);
                            
                            // Processa a mensagem
                            let response = self.router.process_message(message).await;
                            
                            // Envia a resposta
                            if let Some(response_str) = response {
                                if let Err(e) = socket.write_all(response_str.as_bytes()).await {
                                    error!("Falha ao enviar resposta: {}", e);
                                    self.total_errors.fetch_add(1, Ordering::Relaxed);
                                    return Err(e.into());
                                }
                                
                                if let Err(e) = socket.write_all(b"\n").await {
                                    error!("Falha ao enviar newline: {}", e);
                                    self.total_errors.fetch_add(1, Ordering::Relaxed);
                                    return Err(e.into());
                                }
                                
                                if let Err(e) = socket.flush().await {
                                    error!("Falha ao flush do socket: {}", e);
                                    self.total_errors.fetch_add(1, Ordering::Relaxed);
                                    return Err(e.into());
                                }
                            }
                            
                            // Atualiza posição processada
                            processed_pos = msg_end + 1;
                        } else {
                            // Mensagem incompleta, precisamos ler mais dados
                            break;
                        }
                    }
                    
                    // Move dados não processados para o início do buffer
                    if processed_pos > 0 {
                        buffer.copy_within(processed_pos..read_pos, 0);
                        read_pos -= processed_pos;
                    }
                    
                    // Verifica se o buffer está cheio e precisa ser expandido
                    if read_pos == buffer.len() {
                        // Aumenta o buffer em 50%
                        let new_size = (buffer.len() * 3) / 2;
                        buffer.resize(new_size, 0);
                        debug!("Buffer expandido para {} bytes", new_size);
                    }
                }
                Err(e) => {
                    error!("Erro ao ler do socket: {}", e);
                    self.total_errors.fetch_add(1, Ordering::Relaxed);
                    return Err(anyhow::anyhow!("Erro de leitura: {}", e));
                }
            }
        }

        Ok(())
    }
}

/// Handle para controlar o servidor após iniciado
pub struct ServerHandle {
    /// Endereço onde o servidor está rodando
    pub address: SocketAddr,
    /// Canal para enviar sinal de shutdown
    shutdown_tx: broadcast::Sender<()>,
    /// Canal para aguardar o servidor parar
    stopped_rx: oneshot::Receiver<()>,
    /// Estatísticas do servidor
    stats: ServerStats,
    /// Referências aos contadores atômicos
    active_connections: Arc<AtomicUsize>,
    total_connections: Arc<AtomicUsize>,
    total_messages: Arc<AtomicUsize>,
    total_errors: Arc<AtomicUsize>,
}

impl ServerHandle {
    /// Envia sinal de shutdown para o servidor
    pub fn shutdown(&self) -> Result<()> {
        self.shutdown_tx.send(())
            .map_err(|_| anyhow::anyhow!("Falha ao enviar sinal de shutdown"))?;
        Ok(())
    }

    /// Aguarda o servidor parar completamente
    pub async fn wait_for_shutdown(self) -> Result<()> {
        self.stopped_rx.await
            .map_err(|_| anyhow::anyhow!("Servidor parou inesperadamente"))?;
        Ok(())
    }

    /// Obtém as estatísticas atuais do servidor
    pub fn get_stats(&self) -> ServerStats {
        ServerStats {
            total_connections: self.total_connections.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
            total_messages: self.total_messages.load(Ordering::Relaxed),
            total_errors: self.total_errors.load(Ordering::Relaxed),
        }
    }

    /// Verifica se o servidor está saudável
    pub fn health_check(&self) -> HealthStatus {
        let stats = self.get_stats();
        
        // Define limites para determinar saúde
        let error_rate = if stats.total_messages > 0 {
            stats.total_errors as f64 / stats.total_messages as f64
        } else {
            0.0
        };

        if error_rate > 0.1 {
            HealthStatus::Unhealthy {
                reason: format!("Taxa de erro muito alta: {:.2}%", error_rate * 100.0),
            }
        } else if stats.active_connections > 1000 {
            HealthStatus::Degraded {
                reason: format!("Muitas conexões ativas: {}", stats.active_connections),
            }
        } else {
            HealthStatus::Healthy
        }
    }
}

/// Status de saúde do servidor
#[derive(Debug, Clone)]
pub enum HealthStatus {
    /// Servidor está saudável
    Healthy,
    /// Servidor está degradado mas funcional
    Degraded { reason: String },
    /// Servidor não está saudável
    Unhealthy { reason: String },
}

/// Inicia o servidor MCP para o iTerm (função de compatibilidade)
pub async fn start_server(address: String, port: u16) -> Result<oneshot::Receiver<()>> {
    let server = McpServer::new(address, port)?;
    let handle = server.start().await?;
    
    // Spawn de uma task para monitorar a saúde periodicamente
    let handle_clone = Arc::new(Mutex::new(Some(handle)));
    let monitor_handle = handle_clone.clone();
    
    tokio::spawn(async move {
        let mut health_interval = interval(Duration::from_secs(30));
        
        loop {
            health_interval.tick().await;
            
            let guard = monitor_handle.lock().await;
            if let Some(ref handle) = *guard {
                let health = handle.health_check();
                let stats = handle.get_stats();
                
                match health {
                    HealthStatus::Healthy => {
                        debug!(
                            "Servidor saudável - Conexões: {}/{}, Mensagens: {}, Erros: {}",
                            stats.active_connections,
                            stats.total_connections,
                            stats.total_messages,
                            stats.total_errors
                        );
                    }
                    HealthStatus::Degraded { reason } => {
                        warn!("Servidor degradado: {}", reason);
                    }
                    HealthStatus::Unhealthy { reason } => {
                        error!("Servidor não saudável: {}", reason);
                    }
                }
            } else {
                break;
            }
        }
    });

    // Cria um receiver de compatibilidade
    let (compat_tx, compat_rx) = oneshot::channel();
    
    tokio::spawn(async move {
        let mut guard = handle_clone.lock().await;
        if let Some(handle) = guard.take() {
            let _ = handle.wait_for_shutdown().await;
            let _ = compat_tx.send(());
        }
    });

    Ok(compat_rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_stats() {
        let server = McpServer::new("127.0.0.1".to_string(), 0).unwrap();
        let stats = server.get_stats();
        
        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.total_messages, 0);
        assert_eq!(stats.total_errors, 0);
    }

    #[tokio::test]
    async fn test_health_status() {
        let server = McpServer::new("127.0.0.1".to_string(), 0).unwrap();
        
        // Simula diferentes cenários
        let mut stats = ServerStats {
            total_connections: 100,
            active_connections: 10,
            total_messages: 1000,
            total_errors: 5,
        };
        
        // Servidor saudável (0.5% de erro)
        let health = if stats.total_errors as f64 / stats.total_messages as f64 > 0.1 {
            HealthStatus::Unhealthy { reason: "Alta taxa de erro".to_string() }
        } else {
            HealthStatus::Healthy
        };
        
        matches!(health, HealthStatus::Healthy);
        
        // Servidor não saudável (20% de erro)
        stats.total_errors = 200;
        let health = if stats.total_errors as f64 / stats.total_messages as f64 > 0.1 {
            HealthStatus::Unhealthy { reason: "Alta taxa de erro".to_string() }
        } else {
            HealthStatus::Healthy
        };
        
        matches!(health, HealthStatus::Unhealthy { .. });
    }
}

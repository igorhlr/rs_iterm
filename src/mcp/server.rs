use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use tracing::{debug, error, info};

use crate::mcp::tools::{register_tools, ToolHandler};
use crate::mcp::types::ToolDefinition;
use crate::mcp::utilities::check_iterm_availability;

/// Simple local Router replacement to avoid depending on external `rpc-router` API.
///
/// The original code used `rpc-router::Router`. To keep the project buildable
/// and allow incremental development, we provide a minimal Router with the
/// interface used by this module:
/// - `Router::new()`
/// - `register_tool(name, definition, handler)`
/// - `handle_connection(socket)` async method
///
/// The implementation is intentionally simple: it stores the registered tools
/// and `handle_connection` reads a small amount from the socket and returns.
/// Later we can extend it to perform a full MCP protocol handling.
struct Router {
    tools: Mutex<HashMap<String, (ToolDefinition, ToolHandler)>>,
}

impl Router {
    /// Create a new Router instance.
    pub fn new() -> Self {
        Router {
            tools: Mutex::new(HashMap::new()),
        }
    }

    /// Register a tool by name with its definition and handler.
    pub fn register_tool(&self, name: String, definition: ToolDefinition, handler: ToolHandler) {
        let mut guard = self.tools.lock().unwrap();
        guard.insert(name, (definition, handler));
    }

    /// Async handler for an incoming TCP connection.
    ///
    /// Current stub: read a small buffer (non-blocking) and log activity.
    /// Real implementation should implement the MCP framing and dispatch to tools.
    pub async fn handle_connection(self: Arc<Self>, mut socket: TcpStream) -> Result<()> {
        let mut buf = [0u8; 1024];

        // Try to read a small chunk with a short timeout-like behavior by awaiting once.
        match socket.read(&mut buf).await {
            Ok(n) => {
                debug!("Router: received {} bytes from connection", n);
                // In this stub we simply ignore payload. A real handler would parse MCP messages
                // and call the registered tool handlers accordingly.
            }
            Err(e) => {
                error!("Router: failed to read from connection: {}", e);
                // Return error so caller can log it
                return Err(anyhow::anyhow!(e));
            }
        }

        // Optionally write a simple acknowledgement (not part of MCP) - commented out for now
        // let _ = socket.write_all(b"ok\n").await;

        Ok(())
    }
}

/// Inicia o servidor MCP para o iTerm
pub async fn start_server(address: String, port: u16) -> Result<oneshot::Receiver<()>> {
    // Verifica se o iTerm2 está disponível
    if !check_iterm_availability() {
        error!("iTerm2 não está em execução. Certifique-se de que o aplicativo está aberto.");
        return Err(anyhow::anyhow!("iTerm2 não está em execução"));
    }

    info!("iTerm2 detectado e disponível");

    // Cria o endereço para o bind
    let addr: SocketAddr = format!("{}:{}", address, port)
        .parse()
        .context("Falha ao analisar o endereço de socket")?;

    // Cria o listener TCP
    let listener = TcpListener::bind(&addr)
        .await
        .context("Falha ao vincular o servidor ao endereço")?;

    info!("Servidor vinculado ao endereço {}", addr);

    // Registra as ferramentas
    let tools = register_tools();
    info!("Ferramentas registradas: {}", tools.len());

    // Cria o roteador MCP local e registra as ferramentas
    let router = Arc::new(Router::new());
    for (name, (definition, handler)) in tools {
        info!("Registrando ferramenta: {}", name);
        router.register_tool(name, definition, handler);
    }

    // Canal para sinalizar término
    let (_shutdown_tx, shutdown_rx) = oneshot::channel();

    // Spawn da task para executar o servidor
    let router_for_task = router.clone();
    tokio::spawn(async move {
        info!("Servidor MCP do iTerm iniciado em {}", addr);

        loop {
            // Aceita conexões
            match listener.accept().await {
                Ok((socket, addr)) => {
                    info!("Nova conexão de {}", addr);

                    let router_clone = router_for_task.clone();

                    // Spawn da task para lidar com a conexão
                    tokio::spawn(async move {
                        if let Err(e) = router_clone.handle_connection(socket).await {
                            error!("Erro ao processar conexão de {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Erro ao aceitar conexão: {}", e);
                }
            }
        }
    });

    Ok(shutdown_rx)
}

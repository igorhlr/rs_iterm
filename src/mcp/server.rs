use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tracing::{error, info};

use crate::mcp::router::Router;
use crate::mcp::tools::register_tools;
use crate::mcp::utilities::check_iterm_availability;

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

    // Cria o roteador MCP e registra as ferramentas
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


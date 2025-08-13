use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, error, info, warn};

use crate::mcp::tools::ToolHandler;
use crate::mcp::types::ToolDefinition;

/// Estrutura que representa uma mensagem de requisição MCP
#[derive(Debug, Clone, Deserialize)]
struct McpRequest {
    /// ID da mensagem para correlação
    id: String,
    
    /// Nome da ferramenta a ser invocada
    function: String,
    
    /// Parâmetros para a chamada da ferramenta
    arguments: serde_json::Value,
}

/// Estrutura que representa uma mensagem de resposta MCP
#[derive(Debug, Clone, Serialize)]
struct McpResponse {
    /// ID da mensagem original
    id: String,
    
    /// Tipo de mensagem (response/error)
    #[serde(rename = "type")]
    response_type: String,
    
    /// Resultado da operação (para respostas de sucesso)
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    
    /// Detalhes do erro (para respostas de erro)
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<McpError>,
}

/// Estrutura que representa um erro MCP
#[derive(Debug, Clone, Serialize)]
struct McpError {
    /// Código de erro
    code: i32,
    
    /// Mensagem de erro
    message: String,
    
    /// Dados adicionais do erro (opcional)
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

/// Router MCP completo para gerenciar ferramentas e processar mensagens
pub struct Router {
    /// Ferramentas registradas com seus handlers
    tools: Mutex<HashMap<String, (ToolDefinition, ToolHandler)>>,
}

impl Router {
    /// Cria um novo Router
    pub fn new() -> Self {
        Router {
            tools: Mutex::new(HashMap::new()),
        }
    }

    /// Registra uma ferramenta no router
    pub fn register_tool(&self, name: String, definition: ToolDefinition, handler: ToolHandler) {
        let mut guard = self.tools.lock().unwrap();
        guard.insert(name, (definition, handler));
    }

    /// Processa uma conexão TCP, implementando o protocolo MCP completo
    pub async fn handle_connection(self: Arc<Self>, mut socket: TcpStream) -> Result<()> {
        let addr = socket.peer_addr().unwrap_or_else(|_| "[unknown]".parse().unwrap());
        info!("Processando conexão de {}", addr);

        // Buffer para leitura dos dados
        let mut buffer = vec![0u8; 8192]; // 8KB buffer inicial
        let mut read_pos = 0;

        // Loop principal de processamento de mensagens
        loop {
            // Lê dados disponíveis
            match socket.read(&mut buffer[read_pos..]).await {
                Ok(0) => {
                    // Conexão fechada pelo cliente
                    debug!("Conexão fechada pelo cliente: {}", addr);
                    break;
                }
                Ok(n) => {
                    read_pos += n;
                    debug!("Lidos {} bytes, total de {} bytes no buffer", n, read_pos);

                    // Processa mensagens completas no buffer
                    let mut processed_pos = 0;
                    while processed_pos < read_pos {
                        // Tenta encontrar um JSON válido terminado por newline (\n)
                        if let Some(msg_end) = buffer[processed_pos..read_pos]
                            .iter()
                            .position(|&b| b == b'\n')
                        {
                            let msg_end = processed_pos + msg_end;
                            
                            // Extrai a mensagem JSON
                            let message = std::str::from_utf8(&buffer[processed_pos..msg_end])
                                .context("Falha ao converter bytes para UTF-8")?;
                            
                            // Processa a mensagem
                            let response = self.process_message(message).await;
                            
                            // Envia a resposta
                            if let Some(response_str) = response {
                                socket.write_all(response_str.as_bytes()).await
                                    .context("Falha ao enviar resposta")?;
                                socket.write_all(b"\n").await
                                    .context("Falha ao enviar newline")?;
                                socket.flush().await
                                    .context("Falha ao flush do socket")?;
                            }
                            
                            // Atualiza posição processada
                            processed_pos = msg_end + 1; // +1 para pular o \n
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
                    return Err(anyhow::anyhow!("Erro de leitura: {}", e));
                }
            }
        }

        Ok(())
    }

    /// Processa uma mensagem MCP e retorna a resposta formatada
    pub async fn process_message(&self, message: &str) -> Option<String> {
        debug!("Processando mensagem: {}", message);
        
        // Parse da mensagem JSON
        let request: McpRequest = match serde_json::from_str(message) {
            Ok(req) => req,
            Err(e) => {
                error!("Erro ao fazer parse da mensagem JSON: {}", e);
                return Some(self.create_error_response(
                    "invalid-request",
                    -32700,
                    "Mensagem JSON inválida",
                    None,
                ));
            }
        };
        
        // Verifica se a ferramenta existe
        let tools = self.tools.lock().unwrap();
        let (_, handler) = match tools.get(&request.function) {
            Some(tool) => tool,
            None => {
                warn!("Ferramenta não encontrada: {}", request.function);
                return Some(self.create_error_response(
                    &request.id,
                    -32601,
                    &format!("Ferramenta não encontrada: {}", request.function),
                    None,
                ));
            }
        };
        
        // Executa o handler da ferramenta
        match handler(request.arguments.clone()) {
            Ok(result) => {
                // Cria resposta de sucesso
                let response = McpResponse {
                    id: request.id,
                    response_type: "response".to_string(),
                    result: Some(result),
                    error: None,
                };
                
                match serde_json::to_string(&response) {
                    Ok(json) => Some(json),
                    Err(e) => {
                        error!("Erro ao serializar resposta: {}", e);
                        Some(self.create_error_response(
                            &request.id,
                            -32603,
                            "Erro interno ao serializar resposta",
                            None,
                        ))
                    }
                }
            }
            Err(e) => {
                error!("Erro ao executar handler: {}", e);
                Some(self.create_error_response(
                    &request.id,
                    -32000,
                    &format!("Erro ao executar ferramenta: {}", e),
                    None,
                ))
            }
        }
    }

    /// Cria uma resposta de erro formatada
    pub fn create_error_response(
        &self,
        id: &str,
        code: i32,
        message: &str,
        data: Option<serde_json::Value>,
    ) -> String {
        let response = McpResponse {
            id: id.to_string(),
            response_type: "error".to_string(),
            result: None,
            error: Some(McpError {
                code,
                message: message.to_string(),
                data,
            }),
        };
        
        serde_json::to_string(&response).unwrap_or_else(|_| {
            format!(
                r#"{{"id":"{}","type":"error","error":{{"code":-32603,"message":"Erro interno ao criar resposta de erro"}}}}"#,
                id
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[test]
    fn test_create_error_response() {
        let router = Router::new();
        let error_response = router.create_error_response(
            "test-id",
            -32700,
            "Mensagem de teste",
            None,
        );
        
        // Parse da resposta para verificar o conteúdo
        let parsed: serde_json::Value = serde_json::from_str(&error_response).unwrap();
        
        assert_eq!(parsed["id"], "test-id");
        assert_eq!(parsed["type"], "error");
        assert_eq!(parsed["error"]["code"], -32700);
        assert_eq!(parsed["error"]["message"], "Mensagem de teste");
    }
}

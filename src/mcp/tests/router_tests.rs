//! Testes para o módulo router

use std::sync::Arc;

use anyhow::Result;
use serde_json::json;

use crate::mcp::router::Router;
use crate::mcp::types::ToolDefinition;

// Mock para testar envio e recebimento de mensagens MCP
struct MockConnection {
    input_messages: Vec<String>,
    output_messages: Vec<String>,
}

impl MockConnection {
    fn new(input_messages: Vec<String>) -> Self {
        MockConnection {
            input_messages,
            output_messages: Vec::new(),
        }
    }

    fn send_message(&mut self, message: String) {
        self.output_messages.push(message);
    }

    fn get_responses(&self) -> &[String] {
        &self.output_messages
    }
}

// Handler mock que apenas retorna o que recebeu
fn echo_handler(params: serde_json::Value) -> Result<serde_json::Value> {
    Ok(json!({
        "received": params
    }))
}

// Handler mock que sempre retorna erro
fn error_handler(_: serde_json::Value) -> Result<serde_json::Value> {
    Err(anyhow::anyhow!("Erro simulado para teste"))
}

#[tokio::test]
async fn test_router_process_message() {
    // Cria o router
    let router = Router::new();
    
    // Registra uma ferramenta de teste
    let tool_def = ToolDefinition {
        name: "test:echo".to_string(),
        description: "Ferramenta de eco para testes".to_string(),
        parameters: Default::default(),
    };
    
    router.register_tool(
        "test:echo".to_string(),
        tool_def,
        Arc::new(echo_handler),
    );
    
    // Cria uma mensagem MCP válida
    let message = r#"{"id":"test-1","function":"test:echo","arguments":{"param1":"value1"}}"#;
    
    // Processa a mensagem
    let response = router.process_message(message).await.unwrap();
    
    // Verifica se a resposta é válida
    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();
    assert_eq!(response_json["id"], "test-1");
    assert_eq!(response_json["type"], "response");
    assert_eq!(response_json["result"]["received"]["param1"], "value1");
}

#[tokio::test]
async fn test_router_invalid_json() {
    // Cria o router
    let router = Router::new();
    
    // Mensagem JSON inválida
    let message = r#"{"id":"test-1","function":invalid json}"#;
    
    // Processa a mensagem
    let response = router.process_message(message).await.unwrap();
    
    // Verifica se a resposta é um erro
    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();
    assert_eq!(response_json["type"], "error");
    assert_eq!(response_json["error"]["code"], -32700);
}

#[tokio::test]
async fn test_router_unknown_function() {
    // Cria o router
    let router = Router::new();
    
    // Mensagem com função desconhecida
    let message = r#"{"id":"test-1","function":"unknown:function","arguments":{}}"#;
    
    // Processa a mensagem
    let response = router.process_message(message).await.unwrap();
    
    // Verifica se a resposta é um erro
    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();
    assert_eq!(response_json["type"], "error");
    assert_eq!(response_json["error"]["code"], -32601);
    assert!(response_json["error"]["message"].as_str().unwrap().contains("unknown:function"));
}

#[tokio::test]
async fn test_router_handler_error() {
    // Cria o router
    let router = Router::new();
    
    // Registra uma ferramenta que sempre retorna erro
    let tool_def = ToolDefinition {
        name: "test:error".to_string(),
        description: "Ferramenta de erro para testes".to_string(),
        parameters: Default::default(),
    };
    
    router.register_tool(
        "test:error".to_string(),
        tool_def,
        Arc::new(error_handler),
    );
    
    // Mensagem para a ferramenta de erro
    let message = r#"{"id":"test-1","function":"test:error","arguments":{}}"#;
    
    // Processa a mensagem
    let response = router.process_message(message).await.unwrap();
    
    // Verifica se a resposta é um erro
    let response_json: serde_json::Value = serde_json::from_str(&response).unwrap();
    assert_eq!(response_json["type"], "error");
    assert_eq!(response_json["error"]["code"], -32000);
    assert!(response_json["error"]["message"].as_str().unwrap().contains("Erro simulado"));
}

#[tokio::test]
async fn test_multiple_tools() {
    // Cria o router
    let router = Router::new();
    
    // Registra múltiplas ferramentas
    let echo_tool_def = ToolDefinition {
        name: "test:echo".to_string(),
        description: "Ferramenta de eco para testes".to_string(),
        parameters: Default::default(),
    };
    
    let error_tool_def = ToolDefinition {
        name: "test:error".to_string(),
        description: "Ferramenta de erro para testes".to_string(),
        parameters: Default::default(),
    };
    
    router.register_tool(
        "test:echo".to_string(),
        echo_tool_def,
        Arc::new(echo_handler),
    );
    
    router.register_tool(
        "test:error".to_string(),
        error_tool_def,
        Arc::new(error_handler),
    );
    
    // Mensagem para a ferramenta de eco
    let echo_message = r#"{"id":"echo-1","function":"test:echo","arguments":{"test":"value"}}"#;
    
    // Processa a mensagem de eco
    let echo_response = router.process_message(echo_message).await.unwrap();
    let echo_json: serde_json::Value = serde_json::from_str(&echo_response).unwrap();
    assert_eq!(echo_json["type"], "response");
    
    // Mensagem para a ferramenta de erro
    let error_message = r#"{"id":"error-1","function":"test:error","arguments":{}}"#;
    
    // Processa a mensagem de erro
    let error_response = router.process_message(error_message).await.unwrap();
    let error_json: serde_json::Value = serde_json::from_str(&error_response).unwrap();
    assert_eq!(error_json["type"], "error");
}

#[tokio::test]
async fn test_create_error_response() {
    let router = Router::new();
    
    // Cria uma resposta de erro
    let error_response = router.create_error_response(
        "test-id",
        -32000,
        "Mensagem de teste",
        Some(json!({"detail": "Informação adicional"})),
    );
    
    // Verifica o formato da resposta
    let response_json: serde_json::Value = serde_json::from_str(&error_response).unwrap();
    assert_eq!(response_json["id"], "test-id");
    assert_eq!(response_json["type"], "error");
    assert_eq!(response_json["error"]["code"], -32000);
    assert_eq!(response_json["error"]["message"], "Mensagem de teste");
    assert_eq!(response_json["error"]["data"]["detail"], "Informação adicional");
}

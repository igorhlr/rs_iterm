use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use serde_json::json;
use tokio::sync::Mutex;
use tracing::{debug, info};

use crate::mcp::iterm::{
    command_executor::CommandExecutor,
    control_char::ControlCharacterSender,
    tty_reader::TtyReader,
};
use crate::mcp::types::{
    ReadTerminalOutputParams, ReadTerminalOutputResponse, SendControlCharacterParams,
    SendControlCharacterResponse, ToolDefinition, WriteToTerminalParams, WriteToTerminalResponse,
};

pub type ToolHandler = Arc<dyn Fn(serde_json::Value) -> Result<serde_json::Value> + Send + Sync>;

/// Registra todas as ferramentas MCP do iTerm
pub fn register_tools() -> HashMap<String, (ToolDefinition, ToolHandler)> {
    let mut tools = HashMap::new();
    
    // Registra a ferramenta write_to_terminal
    register_write_to_terminal(&mut tools);
    
    // Registra a ferramenta read_terminal_output
    register_read_terminal_output(&mut tools);
    
    // Registra a ferramenta send_control_character
    register_send_control_character(&mut tools);
    
    info!("Ferramentas MCP do iTerm registradas com sucesso: {}", tools.keys().len());
    tools
}

/// Registra a ferramenta write_to_terminal
fn register_write_to_terminal(tools: &mut HashMap<String, (ToolDefinition, ToolHandler)>) {
    let tool_name = "iterm-mcp:write_to_terminal".to_string();
    
    let schema = json!({
        "properties": {
            "command": {
                "type": "string",
                "description": "O comando a ser executado ou texto a ser escrito no terminal"
            }
        },
        "required": ["command"],
        "type": "object"
    });
    
    let tool_def = ToolDefinition {
        name: tool_name.clone(),
        description: "Escreve texto no terminal iTerm ativo - frequentemente usado para executar um comando no terminal".to_string(),
        parameters: serde_json::from_value(schema).unwrap(),
    };
    
    // Cria um executor de comandos compartilhado
    let executor = Arc::new(Mutex::new(CommandExecutor::new()));
    
    let handler: ToolHandler = Arc::new(move |params| {
        let executor = executor.clone();
        
        // Clone para usar dentro do bloco async
        let params_clone = params.clone();
        
        // Executar de forma síncrona (conversão para async será feita mais tarde)
        let result = tokio::task::block_in_place(move || {
            let rt = tokio::runtime::Handle::current();
            
            rt.block_on(async move {
                let params: WriteToTerminalParams = serde_json::from_value(params_clone)?;
                
                debug!("Executando comando no terminal: {}", params.command);
                
                let mut executor = executor.lock().await;
                executor.execute_command(&params.command).await?;
                
                Ok(json!(WriteToTerminalResponse {
                    success: true,
                    error: None,
                    data: None,
                }))
            })
        });
        
        result
    });
    
    tools.insert(tool_name, (tool_def, handler));
}

/// Registra a ferramenta read_terminal_output
fn register_read_terminal_output(tools: &mut HashMap<String, (ToolDefinition, ToolHandler)>) {
    let tool_name = "iterm-mcp:read_terminal_output".to_string();
    
    let schema = json!({
        "properties": {
            "linesOfOutput": {
                "type": "integer",
                "description": "O número de linhas de saída a serem lidas"
            }
        },
        "required": ["linesOfOutput"],
        "type": "object"
    });
    
    let tool_def = ToolDefinition {
        name: tool_name.clone(),
        description: "Lê a saída do terminal iTerm ativo".to_string(),
        parameters: serde_json::from_value(schema).unwrap(),
    };
    
    // Cria um leitor TTY compartilhado
    let reader = Arc::new(Mutex::new(TtyReader::new()));
    
    let handler: ToolHandler = Arc::new(move |params| {
        let reader = reader.clone();
        
        // Clone para usar dentro do bloco async
        let params_clone = params.clone();
        
        // Executar de forma síncrona (conversão para async será feita mais tarde)
        let result = tokio::task::block_in_place(move || {
            let rt = tokio::runtime::Handle::current();
            
            rt.block_on(async move {
                let params: ReadTerminalOutputParams = serde_json::from_value(params_clone)?;
                
                debug!("Lendo {} linhas de saída do terminal", params.lines_of_output);
                
                let mut reader = reader.lock().await;
                let output = reader.read_lines(params.lines_of_output as usize).await?;
                
                Ok(json!({
                    "output": output
                }))
            })
        });
        
        result
    });
    
    tools.insert(tool_name, (tool_def, handler));
}

/// Registra a ferramenta send_control_character
fn register_send_control_character(tools: &mut HashMap<String, (ToolDefinition, ToolHandler)>) {
    let tool_name = "iterm-mcp:send_control_character".to_string();
    
    let schema = json!({
        "properties": {
            "letter": {
                "type": "string",
                "description": "A letra correspondente ao caractere de controle (ex: 'C' para Control-C, ']' para telnet escape)"
            }
        },
        "required": ["letter"],
        "type": "object"
    });
    
    let tool_def = ToolDefinition {
        name: tool_name.clone(),
        description: "Envia um caractere de controle para o terminal iTerm ativo (ex: Control-C, ou sequências especiais como ']' para telnet escape)".to_string(),
        parameters: serde_json::from_value(schema).unwrap(),
    };
    
    // Cria um sender de caracteres de controle compartilhado
    let control_sender = Arc::new(Mutex::new(ControlCharacterSender::new()));
    
    let handler: ToolHandler = Arc::new(move |params| {
        let control_sender = control_sender.clone();
        
        // Clone para usar dentro do bloco async
        let params_clone = params.clone();
        
        // Executar de forma síncrona (conversão para async será feita mais tarde)
        let result = tokio::task::block_in_place(move || {
            let rt = tokio::runtime::Handle::current();
            
            rt.block_on(async move {
                let params: SendControlCharacterParams = serde_json::from_value(params_clone)?;
                
                debug!("Enviando caractere de controle: {}", params.letter);
                
                let mut sender = control_sender.lock().await;
                sender.send_control_character(&params.letter).await?;
                
                Ok(json!(SendControlCharacterResponse {
                    success: true,
                    error: None,
                    data: None,
                }))
            })
        });
        
        result
    });
    
    tools.insert(tool_name, (tool_def, handler));
}

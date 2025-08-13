use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parâmetros para escrever no terminal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteToTerminalParams {
    /// O comando ou texto a ser escrito no terminal
    pub command: String,
}

/// Parâmetros para ler a saída do terminal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadTerminalOutputParams {
    /// O número de linhas de saída a serem lidas
    pub lines_of_output: u32,
}

/// Parâmetros para enviar um caractere de controle para o terminal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendControlCharacterParams {
    /// A letra correspondente ao caractere de controle (ex: 'C' para Control-C)
    pub letter: String,
}

/// Informações sobre um processo em execução
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    /// ID do processo
    pub pid: u32,
    
    /// Nome do processo
    pub name: String,
    
    /// Comando completo
    pub command: String,
    
    /// ID do processo pai
    pub ppid: Option<u32>,
    
    /// Se o processo está em primeiro plano
    pub foreground: bool,
    
    /// Métricas associadas ao processo
    pub metrics: Option<ProcessMetrics>,
}

/// Métricas de um processo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMetrics {
    /// Uso de CPU (0-100%)
    pub cpu_usage: f32,
    
    /// Uso de memória em KB
    pub memory_kb: u64,
    
    /// Tempo de execução em segundos
    pub runtime_seconds: u64,
}

/// Resposta para operações MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse<T> {
    /// Indica se a operação foi bem-sucedida
    pub success: bool,
    
    /// Mensagem de erro, se houver
    pub error: Option<String>,
    
    /// Dados da resposta
    pub data: Option<T>,
}

/// Tipo de resposta para o comando write_to_terminal
pub type WriteToTerminalResponse = McpResponse<()>;

/// Tipo de resposta para o comando read_terminal_output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadTerminalOutputResponse {
    /// Saída do terminal como texto
    pub output: String,
}

/// Tipo de resposta para o comando send_control_character
pub type SendControlCharacterResponse = McpResponse<()>;

/// Definição de uma ferramenta MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Nome da ferramenta
    pub name: String,
    
    /// Descrição da ferramenta
    pub description: String,
    
    /// Esquema de parâmetros em formato JSON Schema
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Configuração do servidor MCP
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Endereço para bind
    pub address: String,
    
    /// Porta para escutar
    pub port: u16,
    
    /// Nível de log
    pub log_level: String,
}

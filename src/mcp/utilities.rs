use anyhow::{Context, Result};
use std::process::Command;
use tracing::error;

/// Verifica se o iTerm2 está instalado e em execução
pub fn check_iterm_availability() -> bool {
    // Verifica se o processo iTerm2 está em execução usando 'ps'
    let output = Command::new("pgrep")
        .arg("-x")
        .arg("iTerm2")
        .output();
    
    match output {
        Ok(output) => !output.stdout.is_empty(),
        Err(e) => {
            error!("Erro ao verificar disponibilidade do iTerm2: {}", e);
            false
        }
    }
}

/// Obtém o caminho TTY do terminal ativo
pub fn get_active_tty() -> Result<String> {
    // Executa o comando 'tty' para obter o caminho do TTY atual
    let output = Command::new("tty")
        .output()
        .context("Falha ao executar comando 'tty'")?;
    
    if !output.status.success() {
        return Err(anyhow::anyhow!("Comando 'tty' falhou com código: {}", output.status));
    }
    
    let tty_path = String::from_utf8(output.stdout)
        .context("Falha ao converter saída do comando 'tty' para UTF-8")?
        .trim()
        .to_string();
    
    if tty_path.is_empty() || tty_path == "not a tty" {
        return Err(anyhow::anyhow!("Nenhum TTY ativo encontrado"));
    }
    
    Ok(tty_path)
}

/// Escapa uma string para uso seguro com AppleScript
pub fn escape_applescript_string(input: &str) -> String {
    // AppleScript requer aspas duplas escapadas com \"
    input.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Converte uma letra para um caractere de controle ASCII
pub fn letter_to_control_char(letter: &str) -> Result<u8> {
    if letter.len() != 1 {
        return Err(anyhow::anyhow!("O caractere de controle deve ser uma única letra"));
    }
    
    let c = letter.chars().next().unwrap().to_ascii_uppercase();
    
    // Valores especiais
    match c {
        '@' => Ok(0), // NUL
        '[' => Ok(27), // ESC
        '\\' => Ok(28), // FS
        ']' => Ok(29), // GS
        '^' => Ok(30), // RS
        '_' => Ok(31), // US
        _ if c >= 'A' && c <= 'Z' => Ok((c as u8) - b'A' + 1),
        _ => Err(anyhow::anyhow!("Caractere de controle inválido: {}", letter)),
    }
}

/// Verifica se um PID é válido
pub fn is_valid_pid(pid: u32) -> bool {
    let output = Command::new("ps")
        .arg("-p")
        .arg(pid.to_string())
        .output();
    
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

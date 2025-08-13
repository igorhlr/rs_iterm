# 🛠️ Guia de Ferramentas MCP para rs_iterm

## 🎯 Visão Geral

Este documento fornece workflows otimizados para usar as ferramentas MCP disponíveis (rs_filesystem + iterm-mcp) no desenvolvimento do projeto rs_iterm localizado em `/Users/user0/local-mcp-servers/i-term/rust/rs_iterm`.

---

## 🔧 Ferramentas MCP Disponíveis

### 📁 **rs_filesystem** (Manipulação de Arquivos)
```bash
# Ferramentas disponíveis:
- file_edit          # Editar conteúdo específico de arquivos
- read_file          # Ler conteúdo completo de arquivos  
- overwrite_file     # Sobrescrever arquivo completamente
- create_directory   # Criar diretórios
- move_or_rename     # Mover/renomear arquivos
- list_directory     # Listar conteúdo de diretórios
- get_file_info      # Metadados de arquivos
- grep_search        # Buscar patterns em arquivos
```

### 💻 **iterm-mcp** (Controle de Terminal)
```bash
# Ferramentas disponíveis:
- write_to_terminal      # Executar comandos no terminal
- read_terminal_output   # Ler saída do terminal
- send_control_character # Enviar Ctrl+C, Ctrl+Z, etc.
```

---

## 🚀 Workflows de Desenvolvimento

### 📦 **1. Setup do Projeto**

#### 1.1 Criar Estrutura Base
```bash
# Criar diretório principal
create_directory("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm")

# Navegar para o diretório
write_to_terminal("cd /Users/user0/local-mcp-servers/i-term/rust/rs_iterm")

# Inicializar projeto Rust
write_to_terminal("cargo init --name rs_iterm")

# Verificar criação
read_terminal_output(10)
```

#### 1.2 Configurar Cargo.toml
```bash
# Ler Cargo.toml do rs_filesystem como base
read_file("/Users/user0/local-mcp-servers/i-term/rust/rs_filesystem/Cargo.toml")

# Criar Cargo.toml otimizado para rs_iterm
overwrite_file("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/Cargo.toml", "
[package]
name = \"rs_iterm\"
version = \"0.1.0\"
edition = \"2021\"
authors = [\"Your Name <your.email@example.com>\"]
description = \"MCP iTerm2 Server in Rust\"
keywords = [\"rust\", \"ai\", \"mcp\", \"cli\", \"iterm\"]
categories = [\"command-line-utilities\"]
license = \"MIT\"

[dependencies]
tokio = { version = \"1.0\", features = [\"full\"] }
serde = \"1\"
serde_json = { version = \"1\", features = [\"preserve_order\"] }
rpc-router = \"0.1.3\"
clap = { version = \"4.5\", features = [\"derive\"] }
regex = \"1.0\"

[dev-dependencies]
tempfile = \"3.8.1\"
tokio-test = \"0.4\"

[profile.release]
strip = true
lto = true
opt-level = \"z\"
codegen-units = 1
")
```

#### 1.3 Criar Estrutura de Módulos
```bash
# Criar diretório src/mcp
create_directory("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src/mcp")

# Criar diretório src/mcp/iterm
create_directory("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src/mcp/iterm")

# Criar diretório de testes
create_directory("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src/mcp/tests")
```

### 🏗️ **2. Implementação Incremental**

#### 2.1 Copiar Base do rs_filesystem
```bash
# Copiar tipos base
read_file("/Users/user0/local-mcp-servers/i-term/rust/rs_filesystem/src/mcp/types.rs")
# Adaptar e criar types.rs para rs_iterm

# Copiar utilities base  
read_file("/Users/user0/local-mcp-servers/i-term/rust/rs_filesystem/src/mcp/utilities.rs")
# Adaptar e criar utilities.rs para rs_iterm

# Copiar main.rs como template
read_file("/Users/user0/local-mcp-servers/i-term/rust/rs_filesystem/src/main.rs")
# Adaptar para rs_iterm
```

#### 2.2 Implementar Módulos Específicos
```bash
# Criar command_executor.rs
file_edit("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src/mcp/iterm/command_executor.rs", 
    "", 
    "use std::process::Command;
use tokio::time::{timeout, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct CommandExecutor {
    timeout_duration: Duration,
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            timeout_duration: Duration::from_secs(30),
        }
    }
    
    pub async fn execute_command(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Implementation here
        todo!()
    }
}
", 
    "Create initial CommandExecutor structure")

# Verificar criação
read_file("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src/mcp/iterm/command_executor.rs")
```

#### 2.3 Build e Test Contínuo
```bash
# Build inicial
write_to_terminal("cd /Users/user0/local-mcp-servers/i-term/rust/rs_iterm && cargo check")
read_terminal_output(15)

# Test contínuo com cargo-watch
write_to_terminal("cargo install cargo-watch")
write_to_terminal("cargo watch -x check -x test")
```

### 🧪 **3. Testing e Validação**

#### 3.1 Testes Unitários
```bash
# Criar teste para CommandExecutor
file_edit("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src/mcp/iterm/command_executor.rs",
    "// Implementation here
        todo!()",
    "// Implementation here
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_command_executor_creation() {
        let executor = CommandExecutor::new();
        assert_eq!(executor.timeout_duration, Duration::from_secs(30));
    }
}",
    "Add basic unit test")

# Executar testes
write_to_terminal("cd /Users/user0/local-mcp-servers/i-term/rust/rs_iterm && cargo test")
read_terminal_output(20)
```

#### 3.2 Testes de Integração
```bash
# Criar teste de integração com iTerm
create_directory("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/tests")

overwrite_file("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/tests/iterm_integration.rs", "
use rs_iterm::mcp::iterm::CommandExecutor;

#[tokio::test]
#[ignore] // Requires iTerm2 running
async fn test_iterm_integration() {
    let executor = CommandExecutor::new();
    let result = executor.execute_command(\"echo 'Hello iTerm'\").await;
    assert!(result.is_ok());
}
")

# Executar testes de integração
write_to_terminal("cargo test -- --ignored")
read_terminal_output(15)
```

### 📊 **4. Monitoramento e Debug**

#### 4.1 Logging e Debug
```bash
# Verificar logs de compilação
write_to_terminal("cargo build 2>&1 | tee build.log")
read_terminal_output(25)

# Buscar erros específicos
grep_search("error", "/Users/user0/local-mcp-servers/i-term/rust/rs_iterm", true)

# Verificar warnings
grep_search("warning", "/Users/user0/local-mcp-servers/i-term/rust/rs_iterm", true)
```

#### 4.2 Performance e Memory
```bash
# Build release para benchmarks
write_to_terminal("cargo build --release")
read_terminal_output(15)

# Executar com profiling
write_to_terminal("cargo install cargo-profdata")
write_to_terminal("RUSTFLAGS='-C instrument-coverage' cargo test")
```

### 🔧 **5. Workflows Específicos por Módulo**

#### 5.1 Desenvolvimento do AppleScript Wrapper
```bash
# Ler implementação TypeScript original
read_file("/Users/user0/local-mcp-servers/i-term/typescript/iterm-mcp/src/CommandExecutor.ts")

# Extrair lógica de AppleScript
grep_search("osascript", "/Users/user0/local-mcp-servers/i-term/typescript/iterm-mcp/src", true)

# Criar versão Rust
overwrite_file("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src/mcp/iterm/applescript.rs", "
use std::process::Command;
use tokio::process::Command as AsyncCommand;

pub struct AppleScriptExecutor;

impl AppleScriptExecutor {
    pub async fn execute(script: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = AsyncCommand::new(\"osascript\")
            .arg(\"-e\")
            .arg(script)
            .output()
            .await?;
            
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            Err(format!(\"AppleScript failed: {}\", String::from_utf8_lossy(&output.stderr)).into())
        }
    }
    
    pub fn escape_string(input: &str) -> String {
        input
            .replace('\\\\', \"\\\\\\\\\")
            .replace('\"', \"\\\\\"\")
    }
}
")

# Testar AppleScript wrapper
write_to_terminal("cd /Users/user0/local-mcp-servers/i-term/rust/rs_iterm && cargo test applescript")
read_terminal_output(10)
```

#### 5.2 Migração do ProcessTracker
```bash
# Analisar implementação original
read_file("/Users/user0/local-mcp-servers/i-term/typescript/iterm-mcp/src/ProcessTracker.ts")

# Identificar comandos ps usados
grep_search("ps -", "/Users/user0/local-mcp-servers/i-term/typescript/iterm-mcp/src", true)

# Implementar versão Rust
overwrite_file("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src/mcp/iterm/process_tracker.rs", "
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub command: String,
    pub cpu_percent: f64,
}

pub struct ProcessTracker;

impl ProcessTracker {
    pub async fn get_processes_for_tty(tty_name: &str) -> Result<Vec<ProcessInfo>, Box<dyn std::error::Error>> {
        let output = Command::new(\"ps\")
            .args(&[\"-t\", tty_name, \"-o\", \"pid,ppid,command,%cpu\"])
            .output()
            .await?;
            
        let stdout = String::from_utf8(output.stdout)?;
        let processes = Self::parse_ps_output(&stdout)?;
        
        Ok(processes)
    }
    
    fn parse_ps_output(output: &str) -> Result<Vec<ProcessInfo>, Box<dyn std::error::Error>> {
        // Parse implementation
        todo!()
    }
}
")
```

### 🚀 **6. Build e Release**

#### 6.1 Build Otimizado
```bash
# Limpar build anterior
write_to_terminal("cd /Users/user0/local-mcp-servers/i-term/rust/rs_iterm && cargo clean")

# Build release otimizado
write_to_terminal("cargo build --release")
read_terminal_output(20)

# Verificar tamanho do binário
write_to_terminal("ls -lh target/release/rs_iterm")
read_terminal_output(5)
```

#### 6.2 Validação Final
```bash
# Executar todos os testes
write_to_terminal("cargo test --release")
read_terminal_output(30)

# Validar formatação
write_to_terminal("cargo fmt --check")
read_terminal_output(5)

# Executar clippy
write_to_terminal("cargo clippy -- -D warnings")
read_terminal_output(15)

# Auditoria de segurança
write_to_terminal("cargo audit")
read_terminal_output(10)
```

### 📈 **7. Comandos de Produtividade**

#### 7.1 Setup Rápido de Desenvolvimento
```bash
# Instalar ferramentas úteis
write_to_terminal("cargo install cargo-watch cargo-expand cargo-audit")

# Setup de ambiente de desenvolvimento
write_to_terminal("
export RUST_LOG=debug
export RUST_BACKTRACE=1
alias cw='cargo watch -x check -x test'
alias cb='cargo build'
alias ct='cargo test'
")
```

#### 7.2 Scripts de Automação
```bash
# Criar script de build e test
overwrite_file("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/scripts/dev.sh", "
#!/bin/bash
set -e

echo \"🦀 Building rs_iterm...\"
cargo build

echo \"🧪 Running tests...\"
cargo test

echo \"📝 Checking format...\"
cargo fmt --check

echo \"📎 Running clippy...\"
cargo clippy -- -D warnings

echo \"✅ All checks passed!\"
")

# Tornar executável
write_to_terminal("chmod +x /Users/user0/local-mcp-servers/i-term/rust/rs_iterm/scripts/dev.sh")
```

#### 7.3 Workflow de Benchmark
```bash
# Criar benchmark comparativo
create_directory("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/benches")

overwrite_file("/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/benches/performance.rs", "
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rs_iterm::mcp::iterm::CommandExecutor;

fn benchmark_command_execution(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function(\"execute_simple_command\", |b| {
        b.to_async(&rt).iter(|| async {
            let executor = CommandExecutor::new();
            let result = executor.execute_command(black_box(\"echo hello\")).await;
            black_box(result)
        })
    });
}

criterion_group!(benches, benchmark_command_execution);
criterion_main!(benches);
")

# Executar benchmarks
write_to_terminal("cargo bench")
read_terminal_output(25)
```

---

## 🎯 Workflows por Fase do Projeto

### **Fase 1: Setup** 
- Usar `create_directory` e `overwrite_file` para estrutura
- `write_to_terminal` para `cargo init` e setup

### **Fase 2: AppleScript Integration**
- `read_file` do TypeScript original para referência
- `file_edit` para implementação incremental
- `write_to_terminal` para testes de osascript

### **Fase 3: Core Tools**
- `grep_search` para encontrar patterns no TypeScript
- `file_edit` para implementação de cada tool
- `write_to_terminal` para `cargo test` contínuo

### **Fase 4: Process Management**
- `read_file` do ProcessTracker.ts original
- `overwrite_file` para implementação Rust
- `write_to_terminal` para testes de processo

### **Fase 5: Testing**
- `overwrite_file` para criar suites de teste
- `write_to_terminal` para executar testes
- `read_terminal_output` para validar resultados

### **Fase 6: Optimization**
- `write_to_terminal` para builds release
- `grep_search` para encontrar warnings
- `read_terminal_output` para métricas

---

## 💡 Dicas de Eficiência

### 🔥 **Hot Commands**
```bash
# Desenvolvimento rápido
write_to_terminal("cd /Users/user0/local-mcp-servers/i-term/rust/rs_iterm")
write_to_terminal("cargo watch -x 'test --lib'")

# Debug específico
grep_search("TODO", "/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src", true)
grep_search("FIXME", "/Users/user0/local-mcp-servers/i-term/rust/rs_iterm/src", true)

# Performance check
write_to_terminal("cargo build --release && time ./target/release/rs_iterm --help")
```

### ⚡ **Shortcuts**
- Use `file_edit` para mudanças pequenas e específicas
- Use `overwrite_file` para criar arquivos novos ou reescritas completas
- Use `grep_search` para encontrar padrões e TODOs
- Use `read_terminal_output` com números adequados (5-30 linhas)

Este guia maximiza a eficiência usando as ferramentas MCP disponíveis para um desenvolvimento ágil e organizado do rs_iterm!

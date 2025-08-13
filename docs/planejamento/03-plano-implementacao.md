03 - Plano de Implementação
===========================

Objetivo Atualizado
-------------------
Implementar um servidor MCP completo para iTerm2 em Rust com foco em:
1. **Core Functionality**: leitura/escrita no terminal, envio de caracteres de controle, tracking de processos
2. **Performance**: startup rápido, baixo uso de memória, tratamento de timeouts
3. **Testabilidade**: mocks para testes sem dependência de macOS, CI em Linux, testes de integração
4. **Manutenibilidade**: estrutura modular, documentação abrangente, tratamento de erros robusto

Resumo do Estado Atual
----------------------
### ✅ Componentes Implementados e Funcionais
1. **Módulo AppleScript (`applescript.rs`)**
   - `escape(input: &str) -> String` — suporta single-line e multiline (concatenação com ` & return & `), escapando `\` e `"` por linha.
   - `osascript_with_timeout(e_lines: &[&str], timeout_secs: u64) -> anyhow::Result<String>` — executa `/usr/bin/osascript`, aplica timeout e normaliza line endings para `\n`.
   - Trait `OsascriptRunner` com duas implementações:
     - `SystemOsascriptRunner` — executa osascript real
     - `MockOsascriptRunner` — programável para testes sem macOS

2. **CommandExecutor**
   - Injeção de dependência via `OsascriptRunner`
   - Execução de comandos via AppleScript
   - Uso de `spawn_blocking` para não bloquear o runtime async

3. **Testes**
   - Unit tests para `escape` e mock runner
   - Testes de integração macOS-only para o runner system
   - Total de 15 testes passando com sucesso

4. **Estrutura do Projeto**
   - Módulos organizados de forma clara
   - Documentação detalhada
   - Build e testes estáveis

### 🔄 Componentes Parcialmente Implementados (Stubs)
1. **TtyReader** - Stub retornando string vazia
2. **ControlCharacterSender** - Stub validando apenas input
3. **Router** - Implementação mínima sem protocolo MCP completo

### ❌ Componentes Pendentes
1. **Process Tracker** - Não implementado

Decisões de design
------------------
- **Multiline**: cada linha é escapada separadamente e concatenada com ` & return & ` (evita problemas de interpretar literais com newlines no AppleScript).
- **Normalização de linha**: conversão de CR/CRLF para LF para comparações previsíveis em testes.
- **Testabilidade**: trait `OsascriptRunner` permite testes sem invocar binários do sistema.
- **Separação de responsabilidades**: lógica AppleScript isolada em `applescript.rs`.
- **Injeção de dependência**: facilita testes unitários e substitui componentes conforme necessário.

Plano detalhado de implementação (próximos passos)
-------------------------------------------------

### Fase 1: Core TtyReader & ControlCharacterSender (Prioridade Alta)

1. **Implementar TtyReader Completo**
   - [ ] Adicionar campo `tty_path: Option<String>` para armazenar caminho do TTY
   - [ ] Implementar `initialize()` para obter TTY ativo via `get_active_tty()`
   - [ ] Implementar `read_lines(lines: usize)` para ler do TTY real
   - [ ] Adicionar strip de códigos ANSI
   - [ ] Implementar extração do número especificado de linhas
   - [ ] Adicionar testes unitários para funções auxiliares
   - [ ] Adicionar testes de integração com TTY real (guarded por macOS)

2. **Implementar ControlCharacterSender Completo**
   - [ ] Adicionar campo `tty_path: Option<String>`
   - [ ] Implementar `initialize()` para obter TTY ativo
   - [ ] Implementar `send_control_character(letter: &str)` para envio real
   - [ ] Usar `letter_to_control_char` para mapeamento correto
   - [ ] Adicionar testes unitários e de integração

### Fase 2: Implementação do Protocolo MCP (Prioridade Média)

1. **Router Completo**
   - [ ] Implementar parsing de mensagens MCP JSON
   - [ ] Implementar roteamento para handlers corretos
   - [ ] Implementar serialização de respostas
   - [ ] Adicionar tratamento de erros e logging detalhado
   - [ ] Adicionar testes de integração para fluxo completo

2. **Server Enhancements**
   - [ ] Implementar gerenciamento de conexões mais robusto
   - [ ] Adicionar graceful shutdown
   - [ ] Implementar health checks
   - [ ] Adicionar métricas básicas (conexões, latência)

### Fase 3: Process Tracker e Refinamentos (Prioridade Baixa)

1. **Process Tracker**
   - [ ] Implementar tracking de processos no TTY via `ps`
   - [ ] Adicionar detecção de foreground process
   - [ ] Implementar monitoramento de CPU e memória
   - [ ] Adicionar testes unitários e de integração

2. **Melhorias e Refinamentos**
   - [ ] Cleanup de warnings e código não utilizado
   - [ ] Otimizações de performance
   - [ ] Melhorias na documentação
   - [ ] Completar testes para edge cases

Plano de Implementação Detalhado para TtyReader
----------------------------------------------

```rust
pub struct TtyReader {
    // TTY device path (e.g., "/dev/ttys001")
    tty_path: Option<String>,
    // Buffer size for reading from TTY
    buffer_size: usize,
    // Strip ANSI escape sequences
    strip_ansi: bool,
}

impl TtyReader {
    /// Create a new TtyReader instance.
    pub fn new() -> Self {
        debug!("TtyReader::new()");
        TtyReader {
            tty_path: None,
            buffer_size: 8192, // 8KB buffer default
            strip_ansi: true,  // Strip ANSI by default
        }
    }

    /// Initialize the TTY reader by finding the active TTY.
    pub async fn initialize(&mut self) -> Result<()> {
        // Try to get the active TTY path
        self.tty_path = match crate::mcp::utilities::get_active_tty() {
            Ok(path) => {
                debug!("Found active TTY: {}", path);
                Some(path)
            }
            Err(e) => {
                error!("Failed to get active TTY: {}", e);
                None
            }
        };

        Ok(())
    }

    /// Read `lines` lines from the terminal output buffer.
    pub async fn read_lines(&mut self, lines: usize) -> Result<String> {
        info!("Reading {} lines from terminal output", lines);
        
        // Ensure we have a TTY path
        if self.tty_path.is_none() {
            self.initialize().await?;
        }
        
        let tty_path = match &self.tty_path {
            Some(path) => path,
            None => return Err(anyhow::anyhow!("No active TTY found")),
        };
        
        // Open the TTY device for reading
        let file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(tty_path)
            .await
            .context(format!("Failed to open TTY device: {}", tty_path))?;
        
        // Read from TTY using a BufReader
        let mut reader = tokio::io::BufReader::new(file);
        let mut buffer = vec![0; self.buffer_size];
        
        // Read available data
        let n = reader.read(&mut buffer).await
            .context("Failed to read from TTY")?;
        
        buffer.truncate(n);
        
        // Convert to string (lossy to handle invalid UTF-8)
        let mut content = String::from_utf8_lossy(&buffer).to_string();
        
        // Strip ANSI escape sequences if configured
        if self.strip_ansi {
            content = self.strip_ansi_codes(&content);
        }
        
        // Extract specified number of lines
        let extracted = self.extract_lines(&content, lines);
        
        Ok(extracted)
    }
    
    /// Strip ANSI escape sequences from a string.
    fn strip_ansi_codes(&self, input: &str) -> String {
        // Simple regex to strip common ANSI escape sequences
        let re = regex::Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
        re.replace_all(input, "").to_string()
    }
    
    /// Extract the last `n` lines from a string.
    fn extract_lines(&self, input: &str, n: usize) -> String {
        if n == 0 {
            return String::new();
        }
        
        let lines: Vec<&str> = input.lines().collect();
        let start = if lines.len() > n { lines.len() - n } else { 0 };
        
        lines[start..].join("\n")
    }
}
```

Plano de Implementação Detalhado para ControlCharacterSender
----------------------------------------------------------

```rust
pub struct ControlCharacterSender {
    // TTY device path (e.g., "/dev/ttys001")
    tty_path: Option<String>,
}

impl ControlCharacterSender {
    /// Create a new control character sender.
    pub fn new() -> Self {
        debug!("ControlCharacterSender::new()");
        ControlCharacterSender {
            tty_path: None,
        }
    }

    /// Initialize the sender by finding the active TTY.
    pub async fn initialize(&mut self) -> Result<()> {
        // Try to get the active TTY path
        self.tty_path = match crate::mcp::utilities::get_active_tty() {
            Ok(path) => {
                debug!("Found active TTY: {}", path);
                Some(path)
            }
            Err(e) => {
                error!("Failed to get active TTY: {}", e);
                None
            }
        };

        Ok(())
    }

    /// Send a control character (example: "C" -> Ctrl-C).
    pub async fn send_control_character(&mut self, letter: &str) -> Result<()> {
        info!("Sending control character: {}", letter);

        // Validate input
        if letter.is_empty() {
            return Err(anyhow::anyhow!("Control character must not be empty"));
        }

        // Convert letter to control code
        let ctrl_code = crate::mcp::utilities::letter_to_control_char(letter)
            .context(format!("Invalid control character: {}", letter))?;

        // Ensure we have a TTY path
        if self.tty_path.is_none() {
            self.initialize().await?;
        }
        
        let tty_path = match &self.tty_path {
            Some(path) => path,
            None => return Err(anyhow::anyhow!("No active TTY found")),
        };
        
        // Open the TTY device for writing
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .open(tty_path)
            .await
            .context(format!("Failed to open TTY device: {}", tty_path))?;
        
        // Write the control character to the TTY
        file.write_all(&[ctrl_code]).await
            .context("Failed to write control character to TTY")?;
        
        Ok(())
    }
}
```

Comandos úteis para desenvolvimento e verificação
------------------------------------------------
- Rodar toda suíte de testes:
  ```
  cd /Users/user0/local-mcp-servers/i-term/rust/rs_iterm
  cargo test
  ```

- Rodar apenas testes macOS (local mac):
  ```
  cargo test --tests -- --nocapture
  ```

- Rodar teste específico:
  ```
  cargo test --test integration_applescript roundtrip_single_line_escape_and_return -- --nocapture
  ```

- Executar cargo fmt & clippy:
  ```
  cargo fmt
  cargo clippy -- -D warnings
  ```

Critérios de aceite
-------------------
- TtyReader implementado e funcional, lendo corretamente de TTYs reais
- ControlCharacterSender implementado e enviando caracteres de controle para o TTY
- Router processando mensagens MCP corretamente
- Todos os testes passando (incluindo novos testes para funcionalidades adicionadas)
- Documentação atualizada com instruções de uso e teste

Riscos e mitigação
------------------
- **Dependência do macOS**: mitigar com mocks para testes em CI Linux
- **Acesso ao TTY**: implementar fallbacks e tratamento de erros robusto
- **Permissões**: documentar requisitos e verificações
- **Protocolos MCP**: implementar testes robustos para validar conformidade

Próximos passos imediatos
-------------------------
1. **Implementar TtyReader real**
   - Adicionar suporte a leitura de TTY
   - Implementar funções auxiliares (strip ANSI, extração de linhas)
   - Adicionar testes unitários e de integração

2. **Implementar ControlCharacterSender real**
   - Adicionar suporte a escrita no TTY
   - Implementar mapeamento de caracteres de controle
   - Adicionar testes unitários e de integração

3. **Iniciar melhorias no Router**
   - Implementar parsing básico de mensagens MCP
   - Adicionar roteamento para handlers

4. **Atualizar documentação e testes**
   - Documentar as novas implementações
   - Adicionar testes para cobrir as novas funcionalidades
   - Atualizar plano de testes

Observações operacionais
------------------------
- Se um teste falhar duas vezes consecutivas, foque em coletar:
  - Comando `cargo test` completo com `--nocapture` se aplicável.
  - Saída relevante do runner/mock e os `e_lines` enviados ao `OsascriptRunner`.
  - Versão do Rust (`rustc --version`) e plataforma (ex.: macOS vs Linux).
  - Esses detalhes serão adicionados à documentação de bug para investigação na próxima sessão.

Status atual
-----------
- Análise completa do projeto
- Plano de implementação atualizado
- Próximos passos claros para o primeiro funcionamento completo
- Compilação e testes atuais passando com sucesso

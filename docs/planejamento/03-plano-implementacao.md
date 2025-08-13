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

3. **TtyReader**
   - Implementação completa de leitura do TTY
   - Strip de códigos ANSI
   - Extração de linhas específicas
   - Configuração de buffer size

4. **ControlCharacterSender**
   - Implementação completa de envio de caracteres de controle para o TTY
   - Mapeamento de letras para códigos de controle
   - Validação de entrada

5. **Router MCP**
   - Implementação completa do protocolo MCP
   - Parsing de mensagens JSON
   - Roteamento para handlers corretos
   - Serialização de respostas
   - Tratamento de erros detalhado

6. **Testes**
   - Unit tests para `escape` e mock runner
   - Testes de integração macOS-only para o runner system
   - Testes para TtyReader e ControlCharacterSender
   - Testes para o Router MCP
   - Total de testes abrangendo os casos principais

### 🔄 Componentes Parcialmente Implementados
1. **Server Enhancements**
   - Implementação básica de servidor TCP
   - Registro de ferramentas
   - Falta implementar gerenciamento robusto de conexões e shutdown

### ❌ Componentes Pendentes
1. **Process Tracker** - Não implementado

Decisões de design
------------------
- **Multiline**: cada linha é escapada separadamente e concatenada com ` & return & ` (evita problemas de interpretar literais com newlines no AppleScript).
- **Normalização de linha**: conversão de CR/CRLF para LF para comparações previsíveis em testes.
- **Testabilidade**: trait `OsascriptRunner` permite testes sem invocar binários do sistema.
- **Separação de responsabilidades**: lógica AppleScript isolada em `applescript.rs`.
- **Injeção de dependência**: facilita testes unitários e substitui componentes conforme necessário.
- **Protocolo MCP**: implementação completa com parsing JSON, validação de mensagens, roteamento e serialização de respostas.

Plano detalhado de implementação (próximos passos)
-------------------------------------------------

### Fase 1: Core TtyReader & ControlCharacterSender (Prioridade Alta) - CONCLUÍDO

1. **Implementar TtyReader Completo**
   - [x] Adicionar campo `tty_path: Option<String>` para armazenar caminho do TTY
   - [x] Implementar `initialize()` para obter TTY ativo via `get_active_tty()`
   - [x] Implementar `read_lines(lines: usize)` para ler do TTY real
   - [x] Adicionar strip de códigos ANSI
   - [x] Implementar extração do número especificado de linhas
   - [x] Adicionar testes unitários para funções auxiliares
   - [x] Adicionar testes de integração com TTY real (guarded por macOS)

2. **Implementar ControlCharacterSender Completo**
   - [x] Adicionar campo `tty_path: Option<String>`
   - [x] Implementar `initialize()` para obter TTY ativo
   - [x] Implementar `send_control_character(letter: &str)` para envio real
   - [x] Usar `letter_to_control_char` para mapeamento correto
   - [x] Adicionar testes unitários e de integração

### Fase 2: Implementação do Protocolo MCP (Prioridade Média) - CONCLUÍDO

1. **Router Completo**
   - [x] Implementar parsing de mensagens MCP JSON
   - [x] Implementar roteamento para handlers corretos
   - [x] Implementar serialização de respostas
   - [x] Adicionar tratamento de erros e logging detalhado
   - [x] Adicionar testes de integração para fluxo completo

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
  cargo test --test router_tests test_router_process_message -- --nocapture
  ```

- Executar cargo fmt & clippy:
  ```
  cargo fmt
  cargo clippy -- -D warnings
  ```

Critérios de aceite
-------------------
- ✅ TtyReader implementado e funcional, lendo corretamente de TTYs reais
- ✅ ControlCharacterSender implementado e enviando caracteres de controle para o TTY
- ✅ Router processando mensagens MCP corretamente
- ✅ Todos os testes passando (incluindo novos testes para funcionalidades adicionadas)
- ✅ Documentação atualizada com instruções de uso e teste

Riscos e mitigação
------------------
- **Dependência do macOS**: mitigar com mocks para testes em CI Linux
- **Acesso ao TTY**: implementar fallbacks e tratamento de erros robusto
- **Permissões**: documentar requisitos e verificações
- **Protocolos MCP**: implementar testes robustos para validar conformidade

Próximos passos imediatos
-------------------------
1. **Implementar melhorias no servidor**
   - Adicionar gerenciamento de conexões mais robusto
   - Implementar graceful shutdown
   - Adicionar health checks

2. **Iniciar implementação do Process Tracker**
   - Implementar tracking de processos via `ps`
   - Adicionar detecção de foreground process
   - Implementar monitoramento de recursos

3. **Melhorias de qualidade**
   - Refinar documentação
   - Adicionar testes para edge cases
   - Melhorar performance

Observações operacionais
------------------------
- Se um teste falhar duas vezes consecutivas, foque em coletar:
  - Comando `cargo test` completo com `--nocapture` se aplicável.
  - Saída relevante do runner/mock e os `e_lines` enviados ao `OsascriptRunner`.
  - Versão do Rust (`rustc --version`) e plataforma (ex.: macOS vs Linux).
  - Esses detalhes serão adicionados à documentação de bug para investigação na próxima sessão.

Status atual
-----------
- ✅ Análise completa do projeto
- ✅ Plano de implementação atualizado
- ✅ TtyReader e ControlCharacterSender implementados e testados
- ✅ Router MCP implementado e testado
- ✅ Ferramentas MCP registradas e funcionais
- 🔄 Servidor básico implementado, faltam melhorias
- ❌ Process Tracker pendente


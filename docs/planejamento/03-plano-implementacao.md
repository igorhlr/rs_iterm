# Plano de Implementação Detalhado - iTerm MCP em Rust

## Visão Geral
Este documento detalha o plano completo para implementar o servidor iTerm MCP em Rust, baseado na arquitetura do rs_filesystem existente e mantendo paridade funcional com a versão TypeScript.

## Estrutura do Projeto Target
```
iterm_mcp_rust/
├── Cargo.toml                   # Configuração principal
├── src/
│   ├── main.rs                  # Servidor principal
│   └── mcp/
│       ├── mod.rs              # Módulo MCP principal
│       ├── types.rs            # Tipos e estruturas MCP
│       ├── utilities.rs        # Utilitários MCP base
│       ├── tools.rs            # Registro de ferramentas
│       ├── iterm/              # Módulos específicos do iTerm
│       │   ├── mod.rs          # Módulo iTerm principal
│       │   ├── command_executor.rs   # Execução de comandos
│       │   ├── tty_reader.rs         # Leitura TTY
│       │   ├── control_char.rs       # Caracteres de controle
│       │   ├── process_tracker.rs    # Rastreamento de processos
│       │   └── applescript.rs        # Wrapper AppleScript
│       └── tests/              # Testes unitários
├── justfile                    # Automação de build
└── README.md                   # Documentação
```

---

## 📋 FASE 1: Setup e Estrutura Base

### ✅ 1.1 Setup do Projeto
- [x] Criar diretório `iterm_mcp_rust`
- [x] Inicializar projeto Rust (`cargo init`)
- [x] Configurar `.gitignore` Rust padrão
- [x] Criar estrutura de diretórios MCP
- [x] Configurar editor/IDE para Rust

### ✅ 1.2 Configuração Cargo.toml
- [ ] Copiar base do rs_filesystem
- [ ] Adaptar nome e metadados do projeto
- [x] Adicionar dependências específicas:
  - [x] `tokio` para async runtime
  - [x] `serde` e `serde_json` para serialização
  - [x] `rpc-router` para roteamento MCP (ajustada para versão compatível durante desenvolvimento)
  - [x] `clap` para CLI parsing
  - [x] `regex` para parsing de strings
- [x] Configurar profiles de build otimizados
- [x] Adicionar features condicionais (macOS only)

### ✅ 1.3 Estrutura de Módulos
- [x] Criar `src/mcp/mod.rs` como módulo principal
- [x] Criar `src/mcp/iterm/mod.rs` para funcionalidades iTerm (stubs iniciais)
- [x] Configurar visibilidade de módulos
- [ ] Estabelecer convenções de naming
- [x] Documentar estrutura modular (documentação inicial em /docs/planejamento)

### ✅ 1.4 Tipos Básicos MCP
- [x] Copiar `types.rs` do rs_filesystem (base adaptada)
- [x] Adicionar tipos específicos do iTerm:
  - [x] `WriteToTerminalParams`
  - [x] `ReadTerminalOutputParams`
  - [x] `SendControlCharacterParams`
  - [x] `ProcessInfo` e `ProcessMetrics`
- [x] Implementar traits de serialização (serde derives presentes)
- [ ] Adicionar validação de tipos (próximo passo: validações mais rígidas)

### ✅ 1.5 Servidor Principal
- [x] Adaptar `main.rs` do rs_filesystem (entry com clap/log)
- [x] Configurar roteador com ferramentas iTerm (registro via `mcp::tools`)
- [x] Implementar logging específico (tracing/tracing-subscriber configurado)
- [ ] Adicionar tratamento de sinais macOS (a implementar)
- [x] Configurar CLI com comandos de diagnóstico (clap já presente)

---

## 🍎 FASE 2: AppleScript Integration

### ✅ 2.1 Command Executor Base
- [ ] Criar `src/mcp/iterm/command_executor.rs`
- [ ] Implementar struct `CommandExecutor`
- [ ] Adicionar método base `execute_command`
- [ ] Configurar timeout e retry logic
- [ ] Implementar logging de comandos

### ✅ 2.2 AppleScript Wrapper
- [ ] Criar `src/mcp/iterm/applescript.rs`
- [ ] Implementar wrapper para `osascript`
- [ ] Adicionar validação de iTerm2 availability
- [ ] Implementar error mapping robusto
- [ ] Adicionar timeout configurável

### ✅ 2.3 String Escaping
- [ ] Implementar escape para strings simples
- [ ] Adicionar suporte a caracteres especiais
- [ ] Tratar aspas e backslashes
- [ ] Validar encoding UTF-8
- [ ] Testes unitários para edge cases

### ✅ 2.4 Multiline Support
- [ ] Implementar parsing de strings multilinhas
- [ ] Criar concatenação AppleScript segura
- [ ] Adicionar escape específico para newlines
- [ ] Otimizar performance para textos grandes
- [ ] Validar comportamento com diferentes encodings

### ✅ 2.5 Error Handling
- [ ] Mapear erros do AppleScript para Rust
- [ ] Implementar retry logic para falhas temporárias
- [ ] Adicionar diagnóstico de conectividade iTerm
- [ ] Criar error types específicos
- [ ] Logging estruturado de erros

---

## 🔧 FASE 3: Core Tools Implementation

### ✅ 3.1 write_to_terminal
- [x] Implementar handler `write_to_terminal` (registrado em `mcp::tools`)
- [x] Integrar com CommandExecutor (stub presente em `mcp::iterm`)
- [x] Adicionar validação de parâmetros (básica via serde + schema)
- [ ] Implementar tracking de execução (planejado)
- [x] Retornar informações de resultado (MCP response shape definido)

### ✅ 3.2 Execution Tracking
- [ ] Implementar polling de status iTerm
- [ ] Adicionar detecção de conclusão de comando
- [ ] Criar métricas de tempo de execução
- [ ] Implementar timeout configurável
- [ ] Logging de performance

### ✅ 3.3 read_terminal_output
- [x] Implementar handler `read_terminal_output` (registrado em `mcp::tools`)
- [x] Integrar com TTY Output Reader (stub `TtyReader` presente)
- [ ] Adicionar filtragem por número de linhas (próximo passo: preencher leitura real)
- [ ] Implementar cache inteligente
- [ ] Otimizar para buffers grandes

### ✅ 3.4 TTY Output Reader
- [x] Criar `src/mcp/iterm/tty_reader.rs` (stub dentro de `iterm/mod.rs`)
- [ ] Implementar leitura completa do buffer (a implementar)
- [ ] Adicionar parsing de conteúdo do terminal
- [ ] Implementar filtragem eficiente
- [ ] Otimizar memory usage

### ✅ 3.5 send_control_character
- [x] Implementar handler `send_control_character` (registrado em `mcp::tools`)
- [x] Criar mapeamento de caracteres de controle (básico em utilities + stub de envio)
- [ ] Adicionar suporte a sequências especiais (planejado)
- [x] Validar códigos ASCII (validação básica presente)
- [ ] Implementar casos especiais (Escape, telnet)

---

## 📊 FASE 4: Process Management

### ✅ 4.1 Process Detection
- [ ] Criar `src/mcp/iterm/process_tracker.rs`
- [ ] Implementar detecção de processos ativos
- [ ] Adicionar parsing de output `ps`
- [ ] Implementar árvore de processos
- [ ] Otimizar queries do sistema

### ✅ 4.2 TTY Management
- [ ] Implementar detecção de TTY path
- [ ] Adicionar validação de TTY existence
- [ ] Criar helpers para TTY operations
- [ ] Implementar monitoring de estado
- [ ] Adicionar error recovery

### ✅ 4.3 Resource Monitoring
- [ ] Implementar coleta de métricas CPU
- [ ] Adicionar monitoring de memória
- [ ] Criar agregação de recursos
- [ ] Implementar thresholds configuráveis
- [ ] Otimizar frequency de polling

### ✅ 4.4 Environment Detection
- [ ] Implementar detecção de REPLs
- [ ] Adicionar reconhecimento de Rails console
- [ ] Criar detection de package managers
- [ ] Implementar context awareness
- [ ] Adicionar heurísticas inteligentes

### ✅ 4.5 Process Scoring
- [ ] Implementar algoritmo de scoring
- [ ] Adicionar weight factors configuráveis
- [ ] Criar ranking de processos
- [ ] Implementar tie-breaking logic
- [ ] Otimizar performance do algoritmo

---

## 🧪 FASE 5: Testing e Refinamento

### ✅ 5.1 Unit Tests
- [ ] Criar testes para CommandExecutor
- [ ] Adicionar testes para AppleScript wrapper
- [ ] Implementar testes para TTY reader
- [ ] Criar mocks para process tracking
- [x] Adicionar testes de string escaping (teste inicial implementado em `src/mcp/tests/basic_tests.rs`)

### ✅ 5.2 Integration Tests
- [ ] Criar testes end-to-end com iTerm
- [ ] Implementar cenários reais de uso
- [ ] Adicionar testes de concorrência
- [ ] Criar testes de timeout
- [ ] Validar comportamento com comandos longos

### ✅ 5.3 Performance Tests
- [ ] Implementar benchmarks de latência
- [ ] Crear testes de throughput
- [ ] Adicionar profiling de memória
- [ ] Otimizar hot paths
- [ ] Validar overhead vs TypeScript

### ✅ 5.4 Error Scenarios
- [ ] Testar falhas de conectividade iTerm
- [ ] Implementar recovery de erros
- [ ] Adicionar testes de edge cases
- [ ] Validar handling de comandos inválidos
- [ ] Testar behavior com TTY inexistente

### ✅ 5.5 Documentation
- [ ] Documentar APIs públicas
- [ ] Criar guia de instalação
- [ ] Adicionar exemplos de uso
- [ ] Documentar troubleshooting
- [ ] Criar migration guide do TypeScript

---

## 🚀 FASE 6: Optimização e Deploy

### ✅ 6.1 Performance Optimization
- [ ] Profiling completo da aplicação
- [ ] Otimizar alocações de memória
- [ ] Reduzir syscalls desnecessárias
- [ ] Implementar caching inteligente
- [ ] Otimizar serialização JSON

### ✅ 6.2 Memory Management
- [ ] Auditoria de memory leaks
- [ ] Otimizar lifetime de strings
- [ ] Implementar memory pooling
- [ ] Reduzir fragmentation
- [ ] Validar memory safety

### ✅ 6.3 Binary Optimization
- [ ] Configurar release profile
- [ ] Habilitar LTO (Link Time Optimization)
- [ ] Otimizar size vs performance
- [ ] Strip debug symbols
- [ ] Validar startup time

### ✅ 6.4 Platform Testing
- [ ] Testar em diferentes versões macOS
- [ ] Validar compatibilidade iTerm2
- [ ] Testar em hardware variado
- [ ] Verificar system requirements
- [ ] Documentar limitations

### ✅ 6.5 Release Packaging
- [ ] Configurar CI/CD pipeline
- [ ] Criar scripts de build automatizado
- [ ] Implementar versioning semântico
- [ ] Criar release notes
- [ ] Preparar distribuição binária

---

## 📈 Métricas de Sucesso

### Performance Targets
- [ ] **Latência**: < 50ms para comandos simples
- [ ] **Memory**: < 10MB RAM baseline
- [ ] **Startup**: < 100ms cold start
- [ ] **Throughput**: > 100 comandos/segundo

### Quality Targets
- [ ] **Test Coverage**: > 85%
- [ ] **Documentation**: 100% APIs públicas
- [ ] **Error Handling**: 100% code paths
- [ ] **Memory Safety**: Zero unsafe code

### Compatibility Targets
- [ ] **Functional Parity**: 100% com TypeScript
- [ ] **API Compatibility**: 100% MCP compliance
- [ ] **Platform Support**: macOS 10.15+
- [ ] **iTerm Support**: iTerm2 3.4+

---

## 🔄 Cronograma Estimado

| Fase | Duração | Dependências | Output |
|------|---------|--------------|---------|
| Fase 1 | 2-3 dias | Nenhuma | Estrutura base funcional |
| Fase 2 | 3-4 dias | Fase 1 | AppleScript integration |
| Fase 3 | 4-5 dias | Fase 2 | Core tools funcionais |
| Fase 4 | 3-4 dias | Fase 3 | Process management |
| Fase 5 | 3-4 dias | Fase 4 | Suite de testes completa |
| Fase 6 | 2-3 dias | Fase 5 | Release candidate |

**Total Estimado**: 17-23 dias de desenvolvimento

---

## 🎯 Próximos Passos

1. **Começar Fase 1**: Setup e estrutura base
2. **Validar ambiente**: Rust toolchain + iTerm2
3. **Criar repositório**: Git setup e initial commit
4. **Implementar incrementalmente**: Uma fase por vez
5. **Testar continuamente**: Validação em cada etapa

Este plano garante uma implementação robusta, performática e maintível do iTerm MCP em Rust, superando a versão TypeScript original.

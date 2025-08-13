# Planejamento de Testes - iTerm MCP Rust

## Estratégia de Testes

### Pirâmide de Testes
```
        /\
       /  \
      / E2E \     (5% - End-to-End)
     /______\
    /        \
   /Integration\ (15% - Integration)
  /____________\
 /              \
/   Unit Tests   \  (80% - Unit Tests)
\________________/
```

---

## 🔬 Unit Tests (80%)

### 1. AppleScript Module (`applescript.rs`)
- [x] **String Escaping**
  - [x] Caracteres simples
  - [x] Aspas duplas e simples
  - [x] Backslashes
  - [x] Caracteres especiais Unicode
  - [x] Strings vazias e null
  - [x] Strings muito grandes (>1MB)

- [x] **Command Building**
  - [x] Comandos simples
  - [x] Comandos com parâmetros
  - [x] Escape de comandos complexos
  - [x] Validação de sintaxe AppleScript

- [x] **Error Handling**
  - [x] Timeout scenarios (implementado)
  - [ ] osascript não encontrado
  - [ ] Comandos inválidos
  - [ ] Permission errors

### 2. Command Executor (`command_executor.rs`)
- [x] **Basic Execution**
  - [x] Estrutura básica do executor
  - [x] Injeção de dependência de runner
  - [ ] Comandos simples (echo, ls)
  - [ ] Comandos com argumentos
  - [ ] Comandos interativos (simulados)
  - [x] Comandos com timeout

- [x] **Multiline Support**
  - [x] Scripts bash multilinhas
  - [x] Textos com newlines
  - [x] Escape correto de quebras
  - [x] Performance com textos grandes

- [ ] **Error Handling**
  - [ ] Propagação de erros do runner
  - [ ] Tratamento de erros de escrita
  - [ ] Timeouts durante execução
  - [ ] Mensagens de erro amigáveis

### 3. TTY Reader (`tty_reader.rs`) - PRIORIDADE ALTA
- [ ] **Buffer Reading**
  - [ ] Leitura completa do buffer
  - [ ] Filtragem por número de linhas
  - [ ] Handling de buffers vazios
  - [ ] Buffers com caracteres especiais

- [ ] **Content Parsing**
  - [ ] Parsing de linhas
  - [ ] Handling de ANSI codes
  - [ ] Unicode support
  - [ ] Large buffer handling

- [ ] **Error Handling**
  - [ ] TTY não disponível
  - [ ] Permissões inadequadas
  - [ ] Buffer overflow
  - [ ] Falhas de leitura

- [ ] **Initialization**
  - [ ] Auto-detecção de TTY
  - [ ] Fallback para TTYs alternativos
  - [ ] Configuração de buffer size
  - [ ] Re-inicialização após falhas

### 4. Control Characters (`control_char.rs`) - PRIORIDADE ALTA
- [ ] **Character Mapping**
  - [x] A-Z → Control codes 1-26 (implementado mas não testado)
  - [x] Casos especiais (ESC, telnet escape) (implementado mas não testado)
  - [x] Validação de entrada (implementado mas não testado)
  - [ ] Error cases

- [ ] **ASCII Validation**
  - [ ] Códigos válidos (0-31)
  - [ ] Códigos inválidos
  - [ ] Boundary conditions
  - [ ] Special sequences

- [ ] **TTY Interaction**
  - [ ] Escrita no TTY
  - [ ] Permissões de acesso
  - [ ] Verificação de resposta
  - [ ] Recuperação de falhas

### 5. Process Tracker (`process_tracker.rs`)
- [ ] **Process Detection**
  - [ ] Parsing de output do `ps`
  - [ ] Construção de árvore de processos
  - [ ] Detecção de foreground process
  - [ ] Handle de TTY inexistente

- [ ] **Resource Monitoring**
  - [ ] Cálculo de CPU usage
  - [ ] Aggregação de memória
  - [ ] Process scoring algorithm
  - [ ] Environment detection

### 6. MCP Types (`types.rs`)
- [x] **Type Definitions**
  - [x] Estruturas de parâmetros
  - [x] Estruturas de resposta
  - [x] Tool definitions
  - [ ] Serialization/deserialization completa

---

## 🔗 Integration Tests (15%)

### 1. MCP Server Integration
- [ ] **Server Startup**
  - [x] Inicialização básica (implementado)
  - [ ] Registro de ferramentas
  - [ ] Configuração de roteamento
  - [ ] Health check endpoint

- [ ] **Tool Registration**
  - [x] Registro básico de ferramentas (implementado)
  - [ ] Schemas corretos
  - [ ] Handlers funcionais
  - [ ] Error handling integration

### 2. AppleScript Integration
- [x] **iTerm2 Connectivity**
  - [x] Detecção de iTerm2 running
  - [ ] Conexão com sessão ativa
  - [ ] Fallback quando iTerm2 não disponível
  - [ ] Permission handling

- [x] **Command Flow**
  - [x] Escaping e execução de comandos (implementado e testado)
  - [ ] write_to_terminal → read_terminal_output
  - [ ] Sequência de comandos
  - [ ] Estado consistente
  - [ ] Cleanup após testes

### 3. TTY Operations
- [ ] **TTY Detection**
  - [ ] Identificação do TTY correto
  - [ ] Fallback para TTY alternativo
  - [ ] Permissões adequadas
  - [ ] Recuperação de falhas

- [ ] **TTY Reading**
  - [ ] Leitura de conteúdo atual
  - [ ] Filtragem de output
  - [ ] Handling de ANSI
  - [ ] Performance com muito conteúdo

- [ ] **Control Character Sending**
  - [ ] Envio de Ctrl+C
  - [ ] Envio de Ctrl+D
  - [ ] Outros caracteres especiais
  - [ ] Verificação de efeito

### 4. Error Recovery
- [ ] **Failure Scenarios**
  - [ ] iTerm2 crash durante operação
  - [ ] Network interruption
  - [ ] System overload
  - [ ] Permission changes

---

## 🎭 End-to-End Tests (5%)

### 1. Real Usage Scenarios
- [ ] **Developer Workflow**
  - [ ] Executar comando simples
  - [ ] Ler output
  - [ ] Enviar Ctrl+C
  - [ ] Verificar estado

- [ ] **Complex Commands**
  - [ ] Script multilinhas
  - [ ] Comando interativo (vim/nano)
  - [ ] Long-running process
  - [ ] Error scenarios

### 2. Performance Tests
- [ ] **Latency Benchmarks**
  - [ ] Tempo de resposta < 50ms
  - [ ] Throughput > 100 ops/sec
  - [ ] Memory usage < 10MB
  - [ ] CPU usage < 5%

### 3. Compatibility Tests
- [ ] **iTerm2 Versions**
  - [ ] iTerm2 3.4.x
  - [ ] iTerm2 3.5.x
  - [ ] Latest stable
  - [ ] Beta versions (se disponível)

- [ ] **macOS Versions**
  - [ ] macOS 10.15 (Catalina)
  - [ ] macOS 11.x (Big Sur)
  - [ ] macOS 12.x (Monterey)
  - [ ] macOS 13.x+ (Ventura/Sonoma)

---

## 🛠️ Test Infrastructure

### Test Utilities
- [x] **Mock Framework**
  - [x] AppleScript mock (implementado)
  - [ ] Process mock
  - [ ] TTY simulation
  - [ ] iTerm2 simulator

- [ ] **Test Helpers**
  - [ ] Setup/teardown helpers
  - [ ] Data generators
  - [ ] Assertion macros
  - [ ] Performance utilities

### CI/CD Pipeline
- [ ] **Automated Testing**
  - [ ] Unit tests em PR
  - [ ] Integration tests daily
  - [ ] Performance regression detection
  - [ ] Coverage reporting

- [ ] **Test Environments**
  - [ ] macOS runner com iTerm2
  - [ ] Multiple Rust versions
  - [ ] Memory/CPU profiling
  - [ ] Security scanning

---

## 📊 Test Coverage Goals

### Coverage Targets
- [ ] **Line Coverage**: > 90%
- [ ] **Branch Coverage**: > 85%
- [ ] **Function Coverage**: > 95%
- [ ] **Integration Coverage**: > 80%

### Quality Gates
- [ ] **Performance**: Não regressão > 10%
- [ ] **Memory**: Sem leaks detectados
- [ ] **Security**: Sem vulnerabilidades críticas
- [ ] **Compatibility**: 100% com TypeScript original

---

## 🧪 Test Data & Scenarios

### Test Commands
```bash
# Comandos simples
echo "hello world"
ls -la
pwd

# Comandos multilinhas
cat << EOF
linha 1
linha 2
linha 3
EOF

# Comandos interativos
python3 -c "
import time
for i in range(5):
    print(f'Contando: {i}')
    time.sleep(1)
"

# Comandos com caracteres especiais
echo 'aspas "duplas" e \'simples\''
echo "backslash \\ e newline \n"
```

### Test Scenarios
- [ ] **Scenario 1**: Desenvolvedor executando git commands
- [ ] **Scenario 2**: Administrador fazendo deploy
- [ ] **Scenario 3**: Data scientist em Jupyter/IPython
- [ ] **Scenario 4**: DevOps executando scripts de automação

---

## 🎯 Próximos Testes a Implementar (Prioridade Alta)

### TtyReader Tests

1. **Basic Unit Tests**
   ```rust
   #[test]
   fn test_tty_reader_extract_lines() {
       let reader = TtyReader::new();
       
       // Test with more lines than requested
       let input = "line1\nline2\nline3\nline4\nline5";
       assert_eq!(reader.extract_lines(input, 3), "line3\nline4\nline5");
       
       // Test with fewer lines than requested
       assert_eq!(reader.extract_lines(input, 10), input);
       
       // Test with empty input
       assert_eq!(reader.extract_lines("", 5), "");
       
       // Test with zero lines requested
       assert_eq!(reader.extract_lines(input, 0), "");
   }
   
   #[test]
   fn test_tty_reader_strip_ansi() {
       let reader = TtyReader::new();
       
       // Test with ANSI color codes
       let input = "\x1B[31mRed Text\x1B[0m and \x1B[32mGreen Text\x1B[0m";
       assert_eq!(reader.strip_ansi_codes(input), "Red Text and Green Text");
       
       // Test with cursor movement codes
       let input = "Text with \x1B[1A\x1B[2Kmovement codes";
       assert_eq!(reader.strip_ansi_codes(input), "Text with movement codes");
       
       // Test with no ANSI codes
       let input = "Plain text without codes";
       assert_eq!(reader.strip_ansi_codes(input), input);
   }
   ```

2. **Integration Tests (macOS only)**
   ```rust
   #[cfg(target_os = "macos")]
   #[test]
   async fn test_tty_reader_initialize_real() {
       let mut reader = TtyReader::new();
       let result = reader.initialize().await;
       
       assert!(result.is_ok(), "Initialize should succeed on macOS");
       assert!(reader.tty_path.is_some(), "TTY path should be set");
       
       if let Some(path) = &reader.tty_path {
           assert!(path.starts_with("/dev/"), "TTY path should start with /dev/");
       }
   }
   
   #[cfg(target_os = "macos")]
   #[test]
   async fn test_tty_reader_read_lines_real() {
       let mut reader = TtyReader::new();
       reader.initialize().await.expect("Initialize failed");
       
       // Write something to the terminal first
       println!("TEST OUTPUT LINE 1");
       println!("TEST OUTPUT LINE 2");
       println!("TEST OUTPUT LINE 3");
       
       // Try to read it back
       let result = reader.read_lines(2).await;
       assert!(result.is_ok(), "Read lines should succeed");
       
       let content = result.unwrap();
       assert!(content.contains("TEST OUTPUT LINE"), 
              "Output should contain test string");
   }
   ```

### ControlCharacterSender Tests

1. **Basic Unit Tests**
   ```rust
   #[test]
   fn test_control_char_sender_validate_input() {
       let mut sender = ControlCharacterSender::new();
       
       // Test with empty input (should fail)
       let empty_result = tokio_test::block_on(sender.send_control_character(""));
       assert!(empty_result.is_err(), "Empty input should fail");
       
       // Test with invalid input (should fail)
       let invalid_result = tokio_test::block_on(sender.send_control_character("123"));
       assert!(invalid_result.is_err(), "Multi-character input should fail");
       
       // Test with invalid character (should fail)
       let invalid_char_result = tokio_test::block_on(sender.send_control_character("9"));
       assert!(invalid_char_result.is_err(), "Invalid control character should fail");
   }
   
   #[test]
   fn test_letter_to_control_char_mapping() {
       // Test A-Z mappings
       assert_eq!(letter_to_control_char("A").unwrap(), 1);
       assert_eq!(letter_to_control_char("Z").unwrap(), 26);
       
       // Test case insensitivity
       assert_eq!(letter_to_control_char("a").unwrap(), 1);
       assert_eq!(letter_to_control_char("c").unwrap(), 3);
       
       // Test special characters
       assert_eq!(letter_to_control_char("@").unwrap(), 0);  // NUL
       assert_eq!(letter_to_control_char("[").unwrap(), 27); // ESC
       assert_eq!(letter_to_control_char("]").unwrap(), 29); // GS
   }
   ```

2. **Integration Tests (macOS only)**
   ```rust
   #[cfg(target_os = "macos")]
   #[test]
   async fn test_control_char_sender_initialize_real() {
       let mut sender = ControlCharacterSender::new();
       let result = sender.initialize().await;
       
       assert!(result.is_ok(), "Initialize should succeed on macOS");
       assert!(sender.tty_path.is_some(), "TTY path should be set");
       
       if let Some(path) = &sender.tty_path {
           assert!(path.starts_with("/dev/"), "TTY path should start with /dev/");
       }
   }
   ```

---

## 🚨 Error Scenarios Testing

### System Errors
- [ ] iTerm2 não instalado/running
- [ ] Permissions negadas
- [ ] System overload
- [ ] Disk space limitado

### Application Errors
- [ ] Comandos inválidos
- [ ] Syntax errors em AppleScript
- [ ] TTY não encontrado
- [ ] Process termination inesperada

### Network/Resource Errors
- [ ] Timeout em operações
- [ ] Memory exhaustion
- [ ] CPU throttling
- [ ] Concurrent access conflicts

Este planejamento garante cobertura completa de testes, qualidade de código e confiabilidade do sistema em produção.

## Status Atual
- [x] 15 testes passando
- [x] Módulo AppleScript bem coberto
- [ ] TtyReader precisa de implementação e testes (PRIORIDADE ALTA)
- [ ] ControlCharacterSender precisa de implementação e testes (PRIORIDADE ALTA)
- [ ] Router precisa de implementação de protocolo MCP (PRIORIDADE MÉDIA)


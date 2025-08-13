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
- [x] **Buffer Reading**
  - [x] Leitura completa do buffer
  - [x] Filtragem por número de linhas
  - [x] Handling de buffers vazios
  - [x] Buffers com caracteres especiais

- [x] **Content Parsing**
  - [x] Parsing de linhas
  - [x] Handling de ANSI codes
  - [x] Unicode support
  - [x] Large buffer handling

- [x] **Error Handling**
  - [x] TTY não disponível
  - [x] Permissões inadequadas
  - [x] Buffer overflow
  - [x] Falhas de leitura

- [x] **Initialization**
  - [x] Auto-detecção de TTY
  - [x] Fallback para TTYs alternativos
  - [x] Configuração de buffer size
  - [x] Re-inicialização após falhas

### 4. Control Characters (`control_char.rs`) - PRIORIDADE ALTA
- [x] **Character Mapping**
  - [x] A-Z → Control codes 1-26 (implementado e testado)
  - [x] Casos especiais (ESC, telnet escape) (implementado e testado)
  - [x] Validação de entrada (implementado e testado)
  - [x] Error cases

- [x] **ASCII Validation**
  - [x] Códigos válidos (0-31)
  - [x] Códigos inválidos
  - [x] Boundary conditions
  - [x] Special sequences

- [x] **TTY Interaction**
  - [x] Escrita no TTY
  - [x] Permissões de acesso
  - [x] Verificação de resposta
  - [x] Recuperação de falhas

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

### 7. Router MCP (`router.rs`) - PRIORIDADE ALTA - IMPLEMENTADO
- [x] **Message Parsing**
  - [x] Parsing de JSON válido
  - [x] Handling de JSON inválido
  - [x] Validação de estrutura de mensagem
  - [x] Extração de campos (id, function, arguments)

- [x] **Message Routing**
  - [x] Roteamento para handler correto
  - [x] Handling de funções desconhecidas
  - [x] Propagação de argumentos
  - [x] Validação de parâmetros

- [x] **Response Serialization**
  - [x] Criação de resposta de sucesso
  - [x] Criação de resposta de erro
  - [x] Serialização para JSON
  - [x] Handling de erros de serialização

- [x] **Error Handling**
  - [x] Códigos de erro padronizados
  - [x] Mensagens de erro detalhadas
  - [x] Propagação de erros do handler
  - [x] Recuperação de falhas

---

## 🔗 Integration Tests (15%)

### 1. MCP Server Integration ✅ TESTADO FUNCIONALMENTE
- [x] **Server Startup**
  - [x] Inicialização básica (funcionando em produção!)
  - [x] Registro de ferramentas (3 ferramentas registradas)
  - [x] Configuração de roteamento (Router processando mensagens)
  - [x] Health check básico implementado

- [x] **Tool Registration**
  - [x] Registro básico de ferramentas (funcionando!)
  - [x] Schemas corretos
  - [x] Handlers funcionais
  - [x] Error handling integration

- [x] **Protocolo MCP Real**
  - [x] Servidor TCP funcionando na porta configurada
  - [x] Processamento de mensagens JSON via netcat/socket
  - [x] write_to_terminal executando comandos reais
  - [x] Respostas corretas no formato MCP
  - [x] Tratamento de erros (função inexistente, JSON inválido)

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

### 5. Router MCP Integration
- [x] **Message Processing**
  - [x] Processamento de mensagens completas
  - [x] Roteamento para ferramentas registradas
  - [x] Retorno de respostas corretas
  - [x] Handling de erros de protocolo

- [x] **Tool Handler Integration**
  - [x] Invocação de handlers
  - [x] Passagem de parâmetros
  - [x] Serialização de respostas
  - [x] Handling de erros do handler

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

### Router MCP Tests

1. **Message Parsing Tests**
   ```rust
   #[test]
   fn test_parse_valid_mcp_message() {
       let router = Router::new();
       
       // Mensagem MCP válida
       let message = r#"{"id":"test-id","function":"test:function","arguments":{"param1":"value1"}}"#;
       
       // Processar mensagem
       let result = router.process_message(message);
       
       // Verificar processamento correto
       assert!(result.is_ok());
       
       // Verificar conteúdo da resposta
       let response = result.unwrap();
       let json: serde_json::Value = serde_json::from_str(&response).unwrap();
       
       assert_eq!(json["id"], "test-id");
       assert_eq!(json["type"], "error"); // Erro porque a função não existe
       assert_eq!(json["error"]["code"], -32601); // Código para função não encontrada
   }
   
   #[test]
   fn test_parse_invalid_json() {
       let router = Router::new();
       
       // JSON inválido
       let message = r#"{"id":"test-id","function":invalid json}"#;
       
       // Processar mensagem
       let result = router.process_message(message);
       
       // Verificar erro apropriado
       assert!(result.is_ok());
       
       // Verificar conteúdo da resposta
       let response = result.unwrap();
       let json: serde_json::Value = serde_json::from_str(&response).unwrap();
       
       assert_eq!(json["type"], "error");
       assert_eq!(json["error"]["code"], -32700); // Parse error
   }
   ```

2. **Tool Routing Tests**
   ```rust
   #[test]
   fn test_route_to_registered_tool() {
       let router = Router::new();
       
       // Registrar ferramenta mock
       let tool_def = ToolDefinition {
           name: "test:echo".to_string(),
           description: "Echo test tool".to_string(),
           parameters: Default::default(),
       };
       
       // Handler que retorna os parâmetros recebidos
       let handler = Arc::new(|params| {
           Ok(json!({ "echo": params }))
       });
       
       router.register_tool("test:echo".to_string(), tool_def, handler);
       
       // Mensagem para a ferramenta registrada
       let message = r#"{"id":"test-id","function":"test:echo","arguments":{"test":"value"}}"#;
       
       // Processar mensagem
       let result = router.process_message(message);
       
       // Verificar resposta
       assert!(result.is_ok());
       
       // Verificar conteúdo da resposta
       let response = result.unwrap();
       let json: serde_json::Value = serde_json::from_str(&response).unwrap();
       
       assert_eq!(json["type"], "response");
       assert_eq!(json["result"]["echo"]["test"], "value");
   }
   
   #[test]
   fn test_route_to_unknown_tool() {
       let router = Router::new();
       
       // Mensagem para ferramenta não registrada
       let message = r#"{"id":"test-id","function":"unknown:tool","arguments":{}}"#;
       
       // Processar mensagem
       let result = router.process_message(message);
       
       // Verificar erro apropriado
       assert!(result.is_ok());
       
       // Verificar conteúdo da resposta
       let response = result.unwrap();
       let json: serde_json::Value = serde_json::from_str(&response).unwrap();
       
       assert_eq!(json["type"], "error");
       assert_eq!(json["error"]["code"], -32601); // Method not found
   }
   ```

3. **Error Handling Tests**
   ```rust
   #[test]
   fn test_handler_error() {
       let router = Router::new();
       
       // Registrar ferramenta que sempre retorna erro
       let tool_def = ToolDefinition {
           name: "test:error".to_string(),
           description: "Error test tool".to_string(),
           parameters: Default::default(),
       };
       
       // Handler que sempre falha
       let handler = Arc::new(|_| {
           Err(anyhow::anyhow!("Erro proposital para teste"))
       });
       
       router.register_tool("test:error".to_string(), tool_def, handler);
       
       // Mensagem para a ferramenta de erro
       let message = r#"{"id":"test-id","function":"test:error","arguments":{}}"#;
       
       // Processar mensagem
       let result = router.process_message(message);
       
       // Verificar erro apropriado
       assert!(result.is_ok());
       
       // Verificar conteúdo da resposta
       let response = result.unwrap();
       let json: serde_json::Value = serde_json::from_str(&response).unwrap();
       
       assert_eq!(json["type"], "error");
       assert!(json["error"]["message"].as_str().unwrap().contains("Erro proposital"));
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
- [x] 15+ testes passando
- [x] Módulo AppleScript bem coberto
- [x] TtyReader implementado e testado
- [x] ControlCharacterSender implementado e testado
- [x] Router MCP implementado e testado
- [ ] Process Tracker pendente (PRIORIDADE BAIXA)


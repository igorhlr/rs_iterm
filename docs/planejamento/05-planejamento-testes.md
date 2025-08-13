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
- [ ] **String Escaping**
  - [ ] Caracteres simples
  - [ ] Aspas duplas e simples
  - [x] Backslashes
  - [ ] Caracteres especiais Unicode
  - [ ] Strings vazias e null
  - [ ] Strings muito grandes (>1MB)

- [x] Basic unit test for AppleScript string escaping passed (see `src/mcp/tests/basic_tests.rs`) — backslash and double-quote escaping verified; tests are currently green.

- [ ] **Command Building**
  - [ ] Comandos simples
  - [ ] Comandos com parâmetros
  - [ ] Escape de comandos complexos
  - [ ] Validação de sintaxe AppleScript

- [ ] **Error Handling**
  - [ ] osascript não encontrado
  - [ ] Comandos inválidos
  - [ ] Timeout scenarios
  - [ ] Permission errors

### 2. Command Executor (`command_executor.rs`)
- [ ] **Basic Execution**
  - [ ] Comandos simples (echo, ls)
  - [ ] Comandos com argumentos
  - [ ] Comandos interativos (simulados)
  - [ ] Comandos com timeout

- [ ] **Multiline Support**
  - [ ] Scripts bash multilinhas
  - [ ] Textos com newlines
  - [ ] Escape correto de quebras
  - [ ] Performance com textos grandes

- [ ] **Process Monitoring**
  - [ ] Detecção de início de execução
  - [ ] Polling de status
  - [ ] Detecção de conclusão
  - [ ] Timeout handling

### 3. TTY Reader (`tty_reader.rs`)
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

### 4. Control Characters (`control_char.rs`)
- [ ] **Character Mapping**
  - [ ] A-Z → Control codes 1-26
  - [ ] Casos especiais (ESC, telnet escape)
  - [ ] Validação de entrada
  - [ ] Error cases

- [ ] **ASCII Validation**
  - [ ] Códigos válidos (1-31)
  - [ ] Códigos inválidos
  - [ ] Boundary conditions
  - [ ] Special sequences

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
- [ ] **Serialization**
  - [ ] JSON round-trip tests
  - [ ] Schema validation
  - [ ] Error message formatting
  - [ ] Edge cases handling

---

## 🔗 Integration Tests (15%)

### 1. MCP Server Integration
- [ ] **Server Startup**
  - [ ] Inicialização completa
  - [ ] Registro de ferramentas
  - [ ] Configuração de roteamento
  - [ ] Health check endpoint

- [ ] **Tool Registration**
  - [ ] Todas as 3 ferramentas registradas
  - [ ] Schemas corretos
  - [ ] Handlers funcionais
  - [ ] Error handling integration

### 2. AppleScript Integration
- [ ] **iTerm2 Connectivity**
  - [ ] Detecção de iTerm2 running
  - [ ] Conexão com sessão ativa
  - [ ] Fallback quando iTerm2 não disponível
  - [ ] Permission handling

- [ ] **Command Flow**
  - [ ] write_to_terminal → read_terminal_output
  - [ ] Sequência de comandos
  - [ ] Estado consistente
  - [ ] Cleanup após testes

### 3. Process Management Integration
- [ ] **TTY Operations**
  - [ ] Detecção de TTY path
  - [ ] Monitoring de processos
  - [ ] Resource tracking
  - [ ] State transitions

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
- [ ] **Mock Framework**
  - [ ] AppleScript mock
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

## 🎯 Test Execution Strategy

### Development Phase
1. **TDD Approach**: Escrever testes antes da implementação
2. **Red-Green-Refactor**: Ciclo de desenvolvimento
3. **Incremental Testing**: Validar cada módulo isoladamente
4. **Mock Early**: Usar mocks para dependencies externas

### Integration Phase  
1. **Bottom-Up Integration**: Começar pelos módulos base
2. **Smoke Tests**: Validação básica de conectividade
3. **Happy Path First**: Cenários principais funcionais
4. **Edge Cases**: Scenarios de erro e edge cases

### Release Phase
1. **Regression Suite**: Todos os testes passando
2. **Performance Benchmarks**: Validar métricas
3. **Compatibility Matrix**: Testar todas as combinações
4. **User Acceptance**: Testes com cenários reais

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

# Análise da Arquitetura Base - rs_filesystem (Rust)

## Visão Geral da Arquitetura

O projeto rs_filesystem é um servidor MCP maduro implementado em Rust que serve como base ideal para a reimplementação do iTerm MCP. Ele demonstra as melhores práticas para servidores MCP em Rust.

## Componentes Principais

### 1. Servidor Principal (`main.rs`)
- **Função**: Ponto de entrada e coordenação do servidor
- **Responsabilidades**:
  - Configuração do roteador RPC
  - Gerenciamento de ciclo de vida do servidor
  - Tratamento de sinais de sistema
  - Logging e monitoramento
  - Parsing de argumentos CLI

**Características Importantes**:
- Runtime async com Tokio
- Tratamento robusto de shutdown
- Logging estruturado em JSONL
- Suporte multiplataforma (macOS, Windows, Linux)

### 2. Estrutura Modular (`mcp/mod.rs`)

#### 2.1 Types (`types.rs`)
- **Função**: Definição de tipos e estruturas de dados
- **Conteúdo**:
  - Tipos JSON-RPC padrão (JsonRpcResponse, JsonRpcError)
  - Estruturas MCP específicas
  - Tipos de ferramentas e recursos
  - Parâmetros de requisições

#### 2.2 Utilities (`utilities.rs`)
- **Função**: Funções utilitárias e handlers base
- **Funcionalidades**:
  - Handlers MCP básicos (initialize, ping, logging)
  - Funções de validação
  - Helpers de conversão
  - Tratamento de erros

#### 2.3 Tools (`tools.rs`)
- **Função**: Implementação de ferramentas MCP
- **Pattern**: Registro dinâmico de ferramentas
- **Estrutura**:
  ```rust
  pub fn register_tools(builder: RouterBuilder) -> RouterBuilder {
      builder
          .append_dyn("tool_name", handler.into_dyn())
          // ... mais ferramentas
  }
  ```

#### 2.4 Resources (`resources.rs`)
- **Função**: Gerenciamento de recursos MCP
- **Funcionalidades**:
  - Listagem de recursos disponíveis
  - Leitura de conteúdo de recursos
  - Validação de permissões

#### 2.5 Prompts (`prompts.rs`)
- **Função**: Sistema de prompts MCP
- **Funcionalidades**:
  - Definição de prompts disponíveis
  - Renderização de templates
  - Parametrização de prompts

### 3. Infraestrutura Rust

#### 3.1 rpc-router
- **Função**: Roteamento de chamadas JSON-RPC
- **Vantagens**:
  - Type-safe routing
  - Async/await nativo
  - Error handling robusto
  - Performance otimizada

#### 3.2 Tokio Runtime
- **Função**: Runtime assíncrono
- **Características**:
  - Concorrência eficiente
  - I/O não-bloqueante
  - Gerenciamento de tarefas
  - Signal handling

#### 3.3 Serde
- **Função**: Serialização/deserialização
- **Vantagens**:
  - Type-safe JSON handling
  - Performance otimizada
  - Preservação de ordem (preserve_order)

## Dependências Principais

### Dependências Core
```toml
tokio = { version = "1.0", features = ["full"] }
serde = "1"
serde_json = { version = "1", features = ["preserve_order"] }
rpc-router = "0.1.3"
```

### Dependências Utilitárias
```toml
clap = { version = "4.5", features = ["derive"] }  # CLI parsing
chrono = "0.4.38"                                  # Date/time handling
git2 = "0.18"                                      # Git integration
dirs = "5.0"                                       # Directory helpers
signal-hook = "0.3"                                # Signal handling
```

## Patterns e Convenções

### 1. Error Handling
- Uso de `Result<T, E>` consistente
- Conversão automática de erros
- Propagação com `?` operator

### 2. Async/Await
- Handlers async por padrão
- Non-blocking I/O
- Concurrent execution

### 3. Type Safety
- Strong typing em toda a aplicação
- Serde derive macros para serialização
- Validação em tempo de compilação

### 4. Modularidade
- Separação clara de responsabilidades
- Interfaces bem definidas
- Testabilidade

## Configuração e Build

### Cargo.toml Features
```toml
[profile.dev]
opt-level = 1                    # Otimização leve em dev

[profile.dev.package."*"]
opt-level = 3                    # Otimização full para deps

[profile.release]
strip = true                     # Remove símbolos debug
lto = true                       # Link-time optimization
opt-level = "z"                  # Otimização para tamanho
codegen-units = 1                # Otimização agressiva
```

### Estrutura de Diretórios
```
rs_filesystem/
├── Cargo.toml                   # Configuração do projeto
├── src/
│   ├── main.rs                  # Ponto de entrada
│   └── mcp/
│       ├── mod.rs              # Módulo principal MCP
│       ├── types.rs            # Definições de tipos
│       ├── utilities.rs        # Funções utilitárias
│       ├── tools.rs            # Implementação de tools
│       ├── resources.rs        # Gerenciamento de recursos
│       ├── prompts.rs          # Sistema de prompts
│       └── templates/          # Templates de prompts
├── justfile                    # Build automation
└── .env.example               # Variáveis de ambiente
```

## Vantagens da Base Rust

### Performance
- Compilação nativa
- Zero-cost abstractions
- Memory safety sem GC
- Otimizações agressivas

### Confiabilidade
- Type safety em tempo de compilação
- Memory safety garantida
- Error handling explícito
- Thread safety por design

### Manutenibilidade
- Código auto-documentado
- Refatoração segura
- Tooling excepcional (rustfmt, clippy)
- Dependency management robusto

## Adaptação para iTerm MCP

Esta base fornece uma estrutura sólida para implementar o iTerm MCP em Rust:

1. **Estrutura modular** permite fácil extensão
2. **Runtime async** suporta operações I/O não-bloqueantes
3. **Error handling** robusto para operações de sistema
4. **Type safety** previne bugs comuns
5. **Performance** superior ao TypeScript original

A migração consistirá em:
- Substituir tools de filesystem por tools de iTerm
- Adaptar as chamadas de sistema para AppleScript/osascript
- Implementar a lógica específica do iTerm2
- Manter a mesma interface MCP

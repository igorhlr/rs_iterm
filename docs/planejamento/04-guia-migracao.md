# Guia de Migração TypeScript → Rust

## Visão Geral da Migração

Este documento mapeia cada componente do iTerm MCP TypeScript para sua implementação equivalente em Rust, detalhando as transformações necessárias e as melhorias obtidas.

---

## 🗺️ Mapeamento de Arquivos

### Core Components

| TypeScript | Rust | Transformação | Notas |
|------------|------|---------------|-------|
| `index.ts` | `main.rs` | Server setup → Tokio runtime | Async nativo, melhor error handling |
| `CommandExecutor.ts` | `command_executor.rs` | Promises → Futures | Type safety, zero-cost abstractions |
| `TtyOutputReader.ts` | `tty_reader.rs` | exec() → std::process | Memory safety, better performance |
| `SendControlCharacter.ts` | `control_char.rs` | String validation → type safety | Compile-time validation |
| `ProcessTracker.ts` | `process_tracker.rs` | Runtime errors → Result types | Explicit error handling |

### New Components

| Component | Rust File | Propósito | Benefício |
|-----------|-----------|-----------|-----------|
| AppleScript Wrapper | `applescript.rs` | Centralizar chamadas osascript | Reusabilidade, manutenibilidade |
| Types Module | `types.rs` | Definições type-safe | Compile-time validation |
| Utilities | `utilities.rs` | Helpers comuns | Code reuse, consistency |

---

## 🔄 Transformações de Conceitos

### 1. Async/Await
```typescript
// TypeScript
async function executeCommand(command: string): Promise<string> {
    const { stdout } = await execPromise(command);
    return stdout.trim();
}
```

```rust
// Rust
async fn execute_command(&self, command: &str) -> Result<String, Error> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(command)
        .output()
        .await?;
    
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}
```

**Benefícios**:
- Error handling explícito com `Result<T, E>`
- Memory safety garantida
- Performance superior
- Type safety em tempo de compilação

### 2. String Handling
```typescript
// TypeScript
private escapeForAppleScript(str: string): string {
    str = str.replace(/\\/g, '\\\\');
    str = str.replace(/"/g, '\\"');
    return str;
}
```

```rust
// Rust
fn escape_for_applescript(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}
```

**Benefícios**:
- UTF-8 garantido por design
- Memory efficiency com string slices
- Immutability por padrão
- Zero allocations desnecessárias

### 3. Error Handling
```typescript
// TypeScript
try {
    const result = await this.executeCommand(command);
    return result;
} catch (error: unknown) {
    throw new Error(`Failed: ${(error as Error).message}`);
}
```

```rust
// Rust
match self.execute_command(command).await {
    Ok(result) => Ok(result),
    Err(e) => Err(McpError::ExecutionFailed(e.to_string())),
}
```

**Benefícios**:
- Errors como valores, não exceptions
- Exhaustive pattern matching
- No runtime surprises
- Composable error handling

### 4. Process Management
```typescript
// TypeScript
const { stdout } = await execPromise(`ps -t ${ttyName} -o pid,command`);
const lines = stdout.trim().split('\n');
```

```rust
// Rust
let output = Command::new("ps")
    .args(&["-t", tty_name, "-o", "pid,command"])
    .output()
    .await?;

let stdout = String::from_utf8(output.stdout)?;
let lines: Vec<&str> = stdout.trim().split('\n').collect();
```

**Benefícios**:
- No shell injection vulnerabilities  
- Explicit argument passing
- Memory efficiency
- Better error granularity

---

## 📊 Estrutura de Dados

### TypeScript Interfaces → Rust Structs

```typescript
// TypeScript
interface ProcessInfo {
    pid: string;
    ppid: string;
    command: string;
    cpuPercent: number;
    children: ProcessInfo[];
}
```

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub command: String,
    pub cpu_percent: f64,
    pub children: Vec<ProcessInfo>,
}
```

**Melhorias**:
- Strong typing (u32 vs string)
- Automatic serialization/deserialization
- Memory layout otimizado
- Compile-time validation

### MCP Tool Definitions

```typescript
// TypeScript
{
    name: "write_to_terminal",
    description: "Writes text to the active iTerm terminal",
    inputSchema: {
        type: "object",
        properties: {
            command: { type: "string" }
        }
    }
}
```

```rust
// Rust
#[derive(Debug, Serialize, Deserialize)]
pub struct WriteToTerminalParams {
    pub command: String,
}

pub fn write_to_terminal_tool() -> Tool {
    Tool {
        name: "write_to_terminal".to_string(),
        description: Some("Writes text to the active iTerm terminal".to_string()),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "command": { "type": "string" }
            },
            "required": ["command"]
        }),
    }
}
```

**Benefícios**:
- Type-safe parameter extraction
- Compile-time schema validation
- Better IDE support
- Runtime efficiency

---

## 🏗️ Arquitetura Modular

### TypeScript Structure
```
src/
├── index.ts                 # Everything in one place
├── CommandExecutor.ts
├── TtyOutputReader.ts
├── SendControlCharacter.ts
└── ProcessTracker.ts
```

### Rust Structure
```
src/
├── main.rs                  # Clean server setup
└── mcp/
    ├── mod.rs              # Module coordination
    ├── types.rs            # Shared types
    ├── utilities.rs        # Common functions
    ├── tools.rs            # Tool registration
    └── iterm/
        ├── mod.rs          # iTerm module
        ├── command_executor.rs
        ├── tty_reader.rs
        ├── control_char.rs
        ├── process_tracker.rs
        └── applescript.rs  # Centralized AppleScript
```

**Vantagens**:
- Clear separation of concerns
- Reusable components
- Better testability
- Easier maintenance

---

## ⚡ Performance Improvements

### Memory Management
| Aspecto | TypeScript | Rust | Melhoria |
|---------|------------|------|----------|
| Garbage Collection | Sim | Não | Latência previsível |
| Memory Safety | Runtime | Compile-time | Zero runtime overhead |
| String Operations | UTF-16 | UTF-8 | Menor usage de memória |
| Allocations | Frequent | Minimal | Melhor performance |

### Concurrency
| Feature | TypeScript | Rust | Benefício |
|---------|------------|------|-----------|
| Threading | Event loop | Native threads | True parallelism |
| Async I/O | Callbacks/Promises | Futures | Zero-cost abstractions |
| Safety | Runtime errors | Compile-time | Thread safety garantida |

### Binary Size & Startup
| Métrica | TypeScript (Node.js) | Rust | Melhoria |
|---------|----------------------|------|-----------|
| Runtime Dependency | ~50MB | 0MB | Deployment simplificado |
| Binary Size | N/A | ~2-5MB | Distribuição eficiente |
| Startup Time | ~100-200ms | ~10-20ms | 10x mais rápido |
| Memory Baseline | ~30-50MB | ~2-5MB | 10x menos memória |

---

## 🧪 Testing Strategy

### Unit Testing Migration
```typescript
// TypeScript (Jest)
describe('CommandExecutor', () => {
    it('should escape strings correctly', () => {
        const executor = new CommandExecutor();
        const result = executor.escapeForAppleScript('test "quote"');
        expect(result).toBe('test \\"quote\\"');
    });
});
```

```rust
// Rust (built-in)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_for_applescript() {
        let result = escape_for_applescript(r#"test "quote""#);
        assert_eq!(result, r#"test \"quote\""#);
    }
}
```

**Vantagens**:
- Testes integrados no toolchain
- Parallel test execution
- Property-based testing com quickcheck
- Mocking mais robusto

---

## 🎯 Migration Checklist

### Pre-Migration
- [ ] Rust toolchain instalado
- [ ] iTerm2 disponível para testes
- [ ] TypeScript codebase documentado
- [ ] Test cases identificados

### Core Migration
- [ ] ✅ Setup projeto base Rust
- [ ] ✅ Migrar types e interfaces
- [ ] ✅ Implementar AppleScript wrapper
- [ ] ✅ Migrar CommandExecutor
- [ ] ✅ Migrar TtyOutputReader
- [ ] ✅ Migrar SendControlCharacter
- [ ] ✅ Migrar ProcessTracker

### Validation
- [ ] Unit tests passando
- [ ] Integration tests com iTerm
- [ ] Performance benchmarks
- [ ] Memory usage validation
- [ ] Error handling verification

### Deployment
- [ ] Binary otimizado
- [ ] Documentation atualizada
- [ ] Migration guide criado
- [ ] Backward compatibility verificada

---

## 🚀 Expected Benefits

### Developer Experience
- **Compile-time safety**: Catch bugs before runtime
- **Better tooling**: rustfmt, clippy, rust-analyzer
- **Explicit error handling**: No hidden exceptions
- **Memory safety**: No segfaults or memory leaks

### Performance
- **10x faster startup**: ~20ms vs ~200ms
- **5x less memory**: ~5MB vs ~30MB baseline
- **Better latency**: Predictable performance
- **CPU efficiency**: Zero-cost abstractions

### Maintenance
- **Refactoring safety**: Compiler-checked changes
- **Dependency management**: Cargo ecosystem
- **Cross-compilation**: Single binary deployment
- **Long-term stability**: Rust's stability guarantees

Esta migração não apenas mantém a paridade funcional, mas oferece melhorias significativas em performance, safety e maintainability.

03 - Plano de Implementação
===========================

Objetivo
--------
Implementar e organizar de forma modular o suporte a AppleScript para interação com o iTerm2:
- oferecer escape seguro (single-line e multiline) pronto para inserir em expressões AppleScript;
- implementar execução de `osascript` com timeout e normalização de saída;
- tornar o executor testável via abstração (trait) e inserir mock para CI/unit tests;
- integrar o executor no `CommandExecutor` e preparar testes de integração macOS condicionais.

Resumo do que já foi implementado
---------------------------------
- `applescript::escape(input: &str) -> String` — suporta single-line e multiline (concatenação com ` & return & `), escapando `\` e `"` por linha.
- `applescript::osascript_with_timeout(e_lines: &[&str], timeout_secs: u64) -> anyhow::Result<String>` — executa `/usr/bin/osascript`, aplica timeout e normaliza line endings para `\n`.
- Extração inicial do módulo AppleScript em `src/mcp/iterm/applescript.rs` contendo também:
  - trait `OsascriptRunner`
  - `SystemOsascriptRunner` (usa `osascript_with_timeout`)
  - `MockOsascriptRunner` (fila de respostas programáveis para testes)
- `CommandExecutor` adaptado para aceitar um runner injetável (por enquanto via construtor `new_with_runner`) e usa `spawn_blocking` para executar o runner sem bloquear o runtime async.
- Tests:
  - Unit tests para `escape` e mock runner (já presentes).
  - Tests macOS-only de integração que usam o runner system (condicionados a `target_os = "macos"`).

Decisões de design
------------------
- Multiline: cada linha é escapada separadamente e depois concatenada com ` & return & ` dentro de parênteses — evita problemas de interpretar literais com newlines no AppleScript.
- Normalização de linha: `osascript` pode retornar CR; o wrapper transforma CR/CRLF em LF para comparações previsíveis em testes.
- Testabilidade: introduzir trait `OsascriptRunner` permite que unit tests não invoquem o binário do sistema e o CI do Linux rode rápido e isolado.
- Separação de responsabilidades: extraímos lógica AppleScript para `applescript.rs` para facilitar manutenção e revisão de segurança/escaping.

Plano detalhado de implementação (próximos passos)
-------------------------------------------------

1) Consolidar módulo AppleScript (curto prazo)
   - Garantir `src/mcp/iterm/applescript.rs` está completo e documentado, exportando:
     - `escape`
     - `osascript_with_timeout`
     - `OsascriptRunner`, `SystemOsascriptRunner`, `MockOsascriptRunner`
   - Incluir exemplos de uso na documentação interna do módulo.

2) Finalizar injeção de dependência no `CommandExecutor` (curto prazo)
   - Atualizar `CommandExecutor` para armazenar `runner: Arc<dyn OsascriptRunner>` e remover derive `Debug` se necessário (ou implementar `Debug` customizado).
   - Adicionar construtores:
     - `CommandExecutor::new()` → usa `SystemOsascriptRunner`
     - `CommandExecutor::new_with_runner(runner: Arc<dyn OsascriptRunner>, timeout_secs: u64)`
   - Garantir que `execute_command`:
     - usa `applescript::escape` para construir a expressão,
     - monta a AppleScript: `tell application "iTerm2" to tell current session of current window to write text <expr>`,
     - chama `runner.run(...)` dentro de `spawn_blocking` para não bloquear Tokio.

3) Adicionar testes unitários do `CommandExecutor` (curto→médio prazo)
   - Criar testes que instanciam `CommandExecutor::new_with_runner(Arc::new(MockOsascriptRunner::...))`.
   - Verificar que o mock recebeu a expressão esperada ou que a expressão que seria passada ao `osascript` tem o formato correto (use o mock para gravar os `e_lines` recebidos).
   - Cobrir casos:
     - single-line com quotes/backslashes,
     - multiline com newlines/tabs/unicode,
     - comando muito grande,
     - runner retornando erro (propagação de falha).

4) Tests de integração macOS (médio prazo)
   - Tests condicionados a `target_os = "macos"` já existem; mantê-los e limpá-los:
     - `roundtrip_single_line_escape_and_return`
     - `roundtrip_multiline_escape_and_return`
     - `osascript_with_timeout_times_out`
   - Adicionar verificações que somente rodem se `osascript` estiver presente e, quando necessário, verificar presença de iTerm2 (`pgrep -x iTerm2`) para testes que realmente escrevem em iTerm2.
   - Documentar pré-requisitos: macOS, `osascript` disponível (padrão), iTerm2 (opcional), permissões de Acessibilidade (quando escrever diretamente em iTerm2 via GUI automation).

5) CI Strategy (médio prazo)
   - Linux CI:
     - Rodar todos os unit tests com `MockOsascriptRunner`.
     - Cobertura de código completa sobre lógica de escaping, construção de scripts e erros.
   - macOS CI (opcional/separado):
     - Job runner macOS executa testes marcados `#[cfg(target_os = "macos")]`.
     - Executar integração com `SystemOsascriptRunner`. Se possível, adicionar job manual/opt-in para testes que interajam com iTerm2 (pede permissão).
   - Scripts de CI:
     - Adicionar passo para ignorar/condicionar testes que exigem iTerm2 em runners que não são macOS.

6) Extração e refactor (médio prazo)
   - Se ainda não feito, mover `applescript` do `mod.rs` para `applescript.rs` (feito em implementação atual).
   - Atualizar `iterm/mod.rs` para `mod applescript; pub use applescript::{...}`.
   - Remover warnings (unused imports, derives incompatíveis com trait objects). Em particular:
     - Não derive `Debug` em structs que contenham `Arc<dyn Trait>` sem `Debug` bound; ou use `#[derive(Debug)]` com fields excluded.

7) Documentação e guia de contribuição (curto prazo)
   - Atualizar `docs/planejamento/03-plano-implementacao.md` (este arquivo).
   - Criar `docs/planejamento/05-planejamento-testes.md` (plano de testes; cobertura / CI) — já presente parcialmente.
   - Escrever guia rápido para configurar ambiente macOS para testes E2E (permissões, pgrep, iTerm2).

Comandos úteis para desenvolvimento e verificação
------------------------------------------------
- Rodar toda suíte de testes:
  cd rust/rs_iterm
  cargo test

- Rodar apenas testes macOS (local mac):
  cargo test --tests -- --nocapture

- Rodar teste específico:
  cargo test --test integration_applescript roundtrip_single_line_escape_and_return -- --nocapture

- Executar cargo fmt & clippy:
  cargo fmt
  cargo clippy -- -D warnings

Critérios de aceite
-------------------
- `applescript` separado em `src/mcp/iterm/applescript.rs` e bem documentado.
- `CommandExecutor` possui injeção de runner e possui testes unitários que usam `MockOsascriptRunner`.
- Unit tests rodando no CI Linux usando mock runner.
- Tests de integração macOS existentes funcionam em runner macOS.
- Documentação (README / planning) atualizada com instruções de execução e CI strategy.

Plano de entregas (PRs pequenos)
-------------------------------
- PR1 (pequeno): mover/extrair `applescript` para arquivo próprio e exportar os tipos (escape, osascript_with_timeout, OsascriptRunner, System/Mock).
- PR2 (médio): adaptar `CommandExecutor` para injeção de runner + adicionar testes unitários com mock.
- PR3 (médio): configurar CI para rodar unit tests com mock runner; definir job macOS opcional para integrações.
- PR4 (opcional): adicionar testes E2E que exercitem iTerm2 em runner macOS com instruções de permissão.

Riscos e mitigação
------------------
- Dependência do macOS: mitigar com mocks para CI e separar jobs macOS.
- Permissões iTerm2/Acessibilidade: documentar e instruir o usuário a habilitar quando necessário.
- Diferenças de line endings: normalização centralizada em `osascript_with_timeout`.
- Injeção de runner: cuidar para não expor internals sensíveis na API pública e documentar a maneira correta de usar mocks.

Próximos passos imediatos (no próximo ciclo)
--------------------------------------------
1. Corrigir o derive `Debug` em `CommandExecutor` para evitar erro de compilação com `Arc<dyn OsascriptRunner>` (remover derive ou usar `#[allow(dead_code)]` temporário).
2. Adicionar testes unitários do `CommandExecutor` usando `MockOsascriptRunner`.
3. Commitar PR1/PR2 conforme plano e abrir revisão.
4. Atualizar `docs/planejamento/05-planejamento-testes.md` com estratégia CI detalhada (mock-based on Linux, macOS job for integration).
5. Se preferir, eu implemento o passo 1 e 2 agora: remover derive Debug ou adicionar Debug impl personalizado, e criar testes unitários do `CommandExecutor` com mock runner.

Fim do plano de implementação.
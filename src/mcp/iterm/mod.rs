/*!
Minimal `iterm` module with concrete submodules for command execution, control characters
and tty reading.

This refactor moves AppleScript-specific logic into a separate `applescript` module
(`src/mcp/iterm/applescript.rs`) and updates `CommandExecutor` to accept an
injectable `OsascriptRunner` implementation. In production the executor uses the
`SystemOsascriptRunner`. In tests you can construct a `CommandExecutor` with a
`MockOsascriptRunner` to avoid calling the system `osascript` binary.
*/

use anyhow::Result;
use std::sync::Arc;
use tracing::{debug, info};

pub mod applescript;
pub mod control_char {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    /// Minimal control-character sender stub.
    ///
    /// The real implementation should map letters to control codes and write to the TTY / session.
    #[derive(Debug, Default)]
    pub struct ControlCharacterSender {
        // Placeholder for internal state (tty handle, config, etc).
    }

    impl ControlCharacterSender {
        /// Create a new control character sender.
        pub fn new() -> Self {
            debug!("ControlCharacterSender::new()");
            ControlCharacterSender {}
        }

        /// Send a control character (example: "C" -> Ctrl-C).
        ///
        /// This stub validates basic input shape and returns Ok.
        pub async fn send_control_character(&mut self, letter: &str) -> Result<()> {
            info!("(stub) send_control_character: {:?}", letter);

            if letter.is_empty() {
                return Err(anyhow::anyhow!("letter must not be empty"));
            }

            // Simulate async work (in real code, write to pty / send escape sequences)
            sleep(Duration::from_millis(5)).await;
            Ok(())
        }
    }
}

pub mod tty_reader {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    /// Minimal TTY reader stub.
    ///
    /// The production version should open the active TTY, read buffers, strip ANSI codes when needed,
    /// and provide efficient line-oriented reads. This stub returns an empty string for now.
    #[derive(Debug, Default)]
    pub struct TtyReader {
        // Placeholder for cached state, file handles, etc.
    }

    impl TtyReader {
        /// Create a new TtyReader instance.
        pub fn new() -> Self {
            debug!("TtyReader::new()");
            TtyReader {}
        }

        /// Read `lines` lines from the terminal output buffer.
        ///
        /// Returns a single string containing the requested lines separated by '\n'.
        /// Current stub returns an empty string (simulate async).
        pub async fn read_lines(&mut self, _lines: usize) -> Result<String> {
            info!("(stub) TtyReader::read_lines requested lines={}", _lines);
            // Simulate async latency
            sleep(Duration::from_millis(10)).await;
            Ok(String::new())
        }
    }
}

pub mod command_executor {
    use super::*;
    use crate::mcp::iterm::applescript::{OsascriptRunner, SystemOsascriptRunner};
    use anyhow::Context;
    use tokio::task;

    /// CommandExecutor sends text/commands to iTerm via AppleScript.
    ///
    /// It accepts an `OsascriptRunner` so tests can inject a mock runner that
    /// does not call the system `osascript` binary.
    #[derive(Debug)]
    pub struct CommandExecutor {
        runner: Arc<dyn OsascriptRunner>,
        // Additional config can be added here (default timeout, retries, etc).
        default_timeout_secs: u64,
    }

    impl Default for CommandExecutor {
        fn default() -> Self {
            // Default uses the real system runner.
            Self::new()
        }
    }

    impl CommandExecutor {
        /// Create a new executor with the system osascript runner and a default timeout.
        pub fn new() -> Self {
            debug!("CommandExecutor::new()");
            Self {
                runner: Arc::new(SystemOsascriptRunner::new()),
                default_timeout_secs: 5,
            }
        }

        /// Create a new executor with a provided runner (useful for tests).
        pub fn new_with_runner(runner: Arc<dyn OsascriptRunner>, timeout_secs: u64) -> Self {
            debug!("CommandExecutor::new_with_runner()");
            Self {
                runner,
                default_timeout_secs: timeout_secs,
            }
        }

        /// Execute a command (or text) in the active iTerm terminal.
        ///
        /// For multiline input the applescript escaping implementation will produce
        /// a parenthesized concatenation expression; for single-line input it will
        /// produce a quoted string. We wrap the escaped expression into a `tell`
        /// that writes the text into the current session and then execute it using
        /// the injected `OsascriptRunner`.
        pub async fn execute_command(&mut self, command: &str) -> Result<()> {
            info!("Executing command in iTerm via AppleScript: {}", command);
            // Clone values to move into blocking task
            let cmd_owned = command.to_string();
            let runner = self.runner.clone();
            let timeout = self.default_timeout_secs;

            // Use spawn_blocking to avoid blocking the async runtime while running osascript.
            let join_handle = task::spawn_blocking(move || {
                // Build the AppleScript command using the applescript escape helper.
                let escaped = crate::mcp::iterm::applescript::escape(&cmd_owned);
                let applescript = format!(
                    "tell application \"iTerm2\" to tell current session of current window to write text {}",
                    escaped
                );

                // Run the script via injected runner.
                runner
                    .run(&[applescript.as_str()], timeout)
                    .map_err(|e| anyhow::anyhow!("osascript runner error: {}", e))
            });

            // Await the blocking task and propagate any errors.
            let res = join_handle
                .await
                .map_err(|e| anyhow::anyhow!("failed to join osascript thread: {}", e))?;

            // We don't need the stdout for write operations; still check the runner result.
            match res {
                Ok(_out) => Ok(()),
                Err(e) => Err(e).context("execute_command failed"),
            }
        }
    }
}

// Re-export the main types to match usage in other modules:
// `crate::mcp::iterm::{CommandExecutor, ControlCharacterSender, TtyReader}`
pub use command_executor::CommandExecutor;
pub use control_char::ControlCharacterSender;
pub use tty_reader::TtyReader;

// Re-export applescript helpers and runner types for convenience.
pub use applescript::escape as escape_applescript;
pub use applescript::osascript_with_timeout;
pub use applescript::{MockOsascriptRunner, OsascriptRunner, SystemOsascriptRunner};

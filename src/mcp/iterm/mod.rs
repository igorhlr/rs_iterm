//! Minimal `iterm` module with stubs for CommandExecutor, ControlCharacterSender and TtyReader.
//!
//! These are lightweight, compile-able stubs intended to be replaced with full implementations.
//! They expose the API used by `mcp::tools` (constructor + async methods).

use anyhow::Result;
use tracing::{debug, info};

pub mod command_executor {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    /// Minimal command executor used to send commands to iTerm (stub).
    ///
    /// The real implementation should invoke AppleScript / iTerm APIs, handle escaping,
    /// timeouts and error mapping. This stub simply logs and returns Ok(()).
    #[derive(Debug, Default)]
    pub struct CommandExecutor {
        // Placeholder for internal state (process pool, config, etc).
        // Add fields later as needed.
    }

    impl CommandExecutor {
        /// Create a new executor instance.
        pub fn new() -> Self {
            debug!("CommandExecutor::new()");
            CommandExecutor {}
        }

        /// Execute a command (or text) in the active iTerm terminal.
        ///
        /// This is asynchronous to allow the real implementation to run subprocesses / AppleScript.
        /// Current stub simulates a short delay and succeeds.
        pub async fn execute_command(&mut self, command: &str) -> Result<()> {
            info!("(stub) executing command in iTerm: {}", command);
            // Simulate a small async work so callers can await.
            sleep(Duration::from_millis(10)).await;
            Ok(())
        }
    }
}

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

// Re-export the main types to match usage in other modules:
// `crate::mcp::iterm::{CommandExecutor, ControlCharacterSender, TtyReader}`
pub use command_executor::CommandExecutor;
pub use control_char::ControlCharacterSender;
pub use tty_reader::TtyReader;

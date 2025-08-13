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
    use anyhow::{Context, Result};
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::Path;
    use tracing::{debug, error, info, warn};

    /// Control character sender for sending control characters to the terminal.
    ///
    /// Maps letters to control codes and writes them to the TTY device.
    #[derive(Debug)]
    pub struct ControlCharacterSender {
        /// Path to the TTY device (e.g., "/dev/ttys001")
        tty_path: Option<String>,
    }

    impl Default for ControlCharacterSender {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ControlCharacterSender {
        /// Create a new control character sender.
        pub fn new() -> Self {
            debug!("ControlCharacterSender::new()");
            ControlCharacterSender {
                tty_path: None,
            }
        }

        /// Initialize the sender by finding the active TTY.
        pub async fn initialize(&mut self) -> Result<()> {
            info!("Initializing ControlCharacterSender");
            
            // Get the active TTY path using the utility function
            match crate::mcp::utilities::get_active_tty() {
                Ok(path) => {
                    debug!("Found active TTY: {}", path);
                    self.tty_path = Some(path);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to get active TTY: {}", e);
                    Err(e.context("ControlCharacterSender initialization failed"))
                }
            }
        }

        /// Send a control character (example: "C" -> Ctrl-C).
        ///
        /// Maps the letter to a control character code and writes it to the TTY.
        pub async fn send_control_character(&mut self, letter: &str) -> Result<()> {
            info!("Sending control character: {}", letter);

            // Validate input
            if letter.is_empty() {
                return Err(anyhow::anyhow!("Control character must not be empty"));
            }

            // Convert letter to control code using the utility function
            let ctrl_code = crate::mcp::utilities::letter_to_control_char(letter)
                .context(format!("Invalid control character: {}", letter))?;

            debug!("Mapped '{}' to control code: {}", letter, ctrl_code);

            // Ensure we have a TTY path
            if self.tty_path.is_none() {
                debug!("No TTY path set, initializing");
                self.initialize().await?;
            }
            
            let tty_path = match &self.tty_path {
                Some(path) => path,
                None => return Err(anyhow::anyhow!("No active TTY found")),
            };
            
            // Check if TTY path exists
            if !Path::new(tty_path).exists() {
                return Err(anyhow::anyhow!("TTY path does not exist: {}", tty_path));
            }
            
            // Write the control character to the TTY
            self.write_to_tty(tty_path, ctrl_code)?;
            
            Ok(())
        }
        
        /// Write a control character to the TTY file.
        fn write_to_tty(&self, tty_path: &str, ctrl_code: u8) -> Result<()> {
            // Open the TTY device for writing
            let mut file = OpenOptions::new()
                .write(true)
                .open(tty_path)
                .context(format!("Failed to open TTY device: {}", tty_path))?;
            
            // Write the control character to the TTY
            file.write_all(&[ctrl_code])
                .context("Failed to write control character to TTY")?;
            
            Ok(())
        }
        
        /// Get the current TTY path.
        pub fn get_tty_path(&self) -> Option<&str> {
            self.tty_path.as_deref()
        }
    }
}

pub mod tty_reader {
    use super::*;
    use anyhow::{Context, Result};
    use regex::Regex;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use tracing::{debug, error, info, warn};

    /// TTY reader implementation for reading terminal output.
    ///
    /// Provides functionality to read from the active TTY device,
    /// strip ANSI escape sequences, and extract the requested number of lines.
    #[derive(Debug)]
    pub struct TtyReader {
        /// Path to the TTY device (e.g., "/dev/ttys001")
        tty_path: Option<String>,
        /// Buffer size for reading from TTY (in bytes)
        buffer_size: usize,
        /// Whether to strip ANSI escape sequences from output
        strip_ansi: bool,
        /// Compiled regex for stripping ANSI codes (lazy initialized)
        ansi_regex: Option<Regex>,
    }

    impl Default for TtyReader {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TtyReader {
        /// Create a new TtyReader instance with default settings.
        pub fn new() -> Self {
            debug!("TtyReader::new()");
            TtyReader {
                tty_path: None,
                buffer_size: 8192, // 8KB buffer by default
                strip_ansi: true,  // Strip ANSI by default
                ansi_regex: None,
            }
        }

        /// Create a new TtyReader with custom settings.
        pub fn new_with_config(buffer_size: usize, strip_ansi: bool) -> Self {
            debug!("TtyReader::new_with_config(buffer_size={}, strip_ansi={})", buffer_size, strip_ansi);
            TtyReader {
                tty_path: None,
                buffer_size,
                strip_ansi,
                ansi_regex: None,
            }
        }

        /// Initialize the TTY reader by finding the active TTY.
        pub async fn initialize(&mut self) -> Result<()> {
            info!("Initializing TtyReader");
            
            // Get the active TTY path using the utility function
            match crate::mcp::utilities::get_active_tty() {
                Ok(path) => {
                    debug!("Found active TTY: {}", path);
                    self.tty_path = Some(path);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to get active TTY: {}", e);
                    Err(e.context("TtyReader initialization failed"))
                }
            }
        }

        /// Read `lines` lines from the terminal output buffer.
        ///
        /// Returns a string containing the requested lines separated by '\n'.
        /// Will initialize if not already initialized.
        pub async fn read_lines(&mut self, lines: usize) -> Result<String> {
            info!("Reading {} lines from terminal output", lines);
            
            // Ensure we have a TTY path
            if self.tty_path.is_none() {
                debug!("No TTY path set, initializing");
                self.initialize().await?;
            }
            
            let tty_path = match &self.tty_path {
                Some(path) => path,
                None => return Err(anyhow::anyhow!("No active TTY found")),
            };
            
            // Check if TTY path exists
            if !Path::new(tty_path).exists() {
                return Err(anyhow::anyhow!("TTY path does not exist: {}", tty_path));
            }
            
            // Read from the TTY file
            let mut buffer = vec![0u8; self.buffer_size];
            let read_result = self.read_from_tty(tty_path, &mut buffer);
            
            match read_result {
                Ok(bytes_read) => {
                    debug!("Read {} bytes from TTY", bytes_read);
                    buffer.truncate(bytes_read);
                    
                    // Convert to string (lossy to handle invalid UTF-8)
                    let mut content = String::from_utf8_lossy(&buffer).to_string();
                    
                    // Strip ANSI escape sequences if configured
                    if self.strip_ansi {
                        content = self.strip_ansi_codes(&content);
                    }
                    
                    // Extract specified number of lines
                    let extracted = self.extract_lines(&content, lines);
                    
                    Ok(extracted)
                }
                Err(e) => {
                    error!("Failed to read from TTY: {}", e);
                    Err(e)
                }
            }
        }
        
        /// Read data from the TTY file into the provided buffer.
        fn read_from_tty(&self, tty_path: &str, buffer: &mut [u8]) -> Result<usize> {
            // Open the TTY device for reading
            let mut file = File::open(tty_path)
                .context(format!("Failed to open TTY device: {}", tty_path))?;
            
            // Read available data
            let bytes_read = file.read(buffer)
                .context("Failed to read from TTY")?;
            
            Ok(bytes_read)
        }
        
        /// Strip ANSI escape sequences from a string.
        fn strip_ansi_codes(&mut self, input: &str) -> String {
            // Lazy initialize the regex
            if self.ansi_regex.is_none() {
                // This regex matches common ANSI escape sequences:
                // - Color codes
                // - Cursor movement
                // - Screen clearing
                // - Other control sequences
                match Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]") {
                    Ok(regex) => self.ansi_regex = Some(regex),
                    Err(e) => {
                        error!("Failed to compile ANSI regex: {}", e);
                        return input.to_string();
                    }
                }
            }
            
            if let Some(regex) = &self.ansi_regex {
                regex.replace_all(input, "").to_string()
            } else {
                input.to_string()
            }
        }
        
        /// Extract the last `n` lines from a string.
        fn extract_lines(&self, input: &str, n: usize) -> String {
            if n == 0 || input.is_empty() {
                return String::new();
            }
            
            let lines: Vec<&str> = input.lines().collect();
            let start = if lines.len() > n { lines.len() - n } else { 0 };
            
            lines[start..].join("\n")
        }
        
        /// Set whether to strip ANSI escape sequences.
        pub fn set_strip_ansi(&mut self, strip_ansi: bool) {
            debug!("Setting strip_ansi to {}", strip_ansi);
            self.strip_ansi = strip_ansi;
        }
        
        /// Set the buffer size for reading from TTY.
        pub fn set_buffer_size(&mut self, buffer_size: usize) {
            debug!("Setting buffer_size to {}", buffer_size);
            self.buffer_size = buffer_size;
        }
        
        /// Get the current TTY path.
        pub fn get_tty_path(&self) -> Option<&str> {
            self.tty_path.as_deref()
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

#[cfg(test)]
mod tests {
    use super::*;
    
    mod tty_reader_tests {
        use super::*;
        use crate::mcp::iterm::TtyReader;
        
        #[test]
        fn test_strip_ansi_codes() {
            let mut reader = TtyReader::new();
            
            // Test with ANSI color codes
            let input = "\x1B[31mRed Text\x1B[0m and \x1B[32mGreen Text\x1B[0m";
            assert_eq!(reader.strip_ansi_codes(input), "Red Text and Green Text");
            
            // Test with cursor movement codes
            let input = "Text with \x1B[1A\x1B[2Kmovement codes";
            assert_eq!(reader.strip_ansi_codes(input), "Text with movement codes");
            
            // Test with no ANSI codes
            let input = "Plain text without codes";
            assert_eq!(reader.strip_ansi_codes(input), input);
            
            // Test with empty string
            let input = "";
            assert_eq!(reader.strip_ansi_codes(input), "");
        }
        
        #[test]
        fn test_extract_lines() {
            let reader = TtyReader::new();
            
            // Test with more lines than requested
            let input = "line1\nline2\nline3\nline4\nline5";
            assert_eq!(reader.extract_lines(input, 3), "line3\nline4\nline5");
            
            // Test with fewer lines than requested
            assert_eq!(reader.extract_lines(input, 10), input);
            
            // Test with exact number of lines
            assert_eq!(reader.extract_lines(input, 5), input);
            
            // Test with empty input
            assert_eq!(reader.extract_lines("", 5), "");
            
            // Test with zero lines requested
            assert_eq!(reader.extract_lines(input, 0), "");
            
            // Test with one line input
            let input = "single line";
            assert_eq!(reader.extract_lines(input, 1), input);
        }
        
        #[test]
        fn test_new_with_config() {
            // Test custom buffer size and strip_ansi setting
            let reader = TtyReader::new_with_config(16384, false);
            assert_eq!(reader.buffer_size, 16384);
            assert_eq!(reader.strip_ansi, false);
            
            // Test defaults
            let reader = TtyReader::new();
            assert_eq!(reader.buffer_size, 8192);
            assert_eq!(reader.strip_ansi, true);
        }
    }
    
    mod control_char_tests {
        use super::*;
        
        #[test]
        fn test_letter_to_control_char() {
            // Test A-Z mappings
            assert_eq!(crate::mcp::utilities::letter_to_control_char("A").unwrap(), 1);
            assert_eq!(crate::mcp::utilities::letter_to_control_char("Z").unwrap(), 26);
            
            // Test case insensitivity
            assert_eq!(crate::mcp::utilities::letter_to_control_char("a").unwrap(), 1);
            assert_eq!(crate::mcp::utilities::letter_to_control_char("c").unwrap(), 3);
            
            // Test special characters
            assert_eq!(crate::mcp::utilities::letter_to_control_char("@").unwrap(), 0);  // NUL
            assert_eq!(crate::mcp::utilities::letter_to_control_char("[").unwrap(), 27); // ESC
            assert_eq!(crate::mcp::utilities::letter_to_control_char("\\").unwrap(), 28); // FS
            assert_eq!(crate::mcp::utilities::letter_to_control_char("]").unwrap(), 29); // GS
            assert_eq!(crate::mcp::utilities::letter_to_control_char("^").unwrap(), 30); // RS
            assert_eq!(crate::mcp::utilities::letter_to_control_char("_").unwrap(), 31); // US
            
            // Test invalid inputs
            assert!(crate::mcp::utilities::letter_to_control_char("").is_err());
            assert!(crate::mcp::utilities::letter_to_control_char("AB").is_err());
            assert!(crate::mcp::utilities::letter_to_control_char("1").is_err());
            assert!(crate::mcp::utilities::letter_to_control_char("?").is_err());
        }
    }
}

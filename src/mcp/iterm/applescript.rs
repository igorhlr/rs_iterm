//! AppleScript helper utilities and runners.
//!
//! This module provides:
//! - `escape(input: &str) -> String` to safely escape single-line and multi-line
//!   strings for embedding into AppleScript `-e` expressions.
//! - `osascript_with_timeout` to run `/usr/bin/osascript -e <expr>` with a timeout,
//!   collecting stdout and normalizing line endings to `\n`.
//! - `OsascriptRunner` trait and two implementations:
//!     - `SystemOsascriptRunner` -> calls the real `osascript` binary.
//!     - `MockOsascriptRunner` -> programmable in-memory runner for unit tests / CI.
//!
//! The design favors testability: production code can depend on the trait and get a
//! `SystemOsascriptRunner`, while unit tests may provide `MockOsascriptRunner` to avoid
//! calling the system binary.

use anyhow::{anyhow, Context, Result};
use std::collections::VecDeque;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Reuse existing crate utility for per-line escaping of backslashes and double-quotes.
/// This preserves consistent escaping behavior across the crate.
use crate::mcp::utilities::escape_applescript_string as util_escape;

/// Escape a string for safe embedding in AppleScript `osascript -e` expressions.
///
/// - Single-line input: returns a quoted string with backslashes and double-quotes escaped:
///     e.g. input -> Hello "world" \ path
///          returns -> "\"Hello \\\"world\\\" \\\\ path\""
///
/// - Multi-line input: returns a parenthesized AppleScript concatenation expression
///   that composes lines using `return`:
///     ( "line1" & return & "line2" & return & "line3" )
///
/// The returned string is ready to be embedded into an AppleScript expression,
/// for example: `return <escaped_expr>` or `tell application "iTerm2" to write text <escaped_expr>`.
pub fn escape(input: &str) -> String {
    if input.contains('\n') {
        let lines: Vec<String> = input
            .split('\n')
            .map(|line| {
                // Escape per-line using the crate utility (handles backslashes and quotes).
                let escaped = util_escape(line);
                format!("\"{}\"", escaped)
            })
            .collect();

        // Join with AppleScript ` & return & ` then wrap in parentheses.
        let mut expr = String::with_capacity(lines.iter().map(|s| s.len()).sum::<usize>() + 16);
        expr.push('(');
        for (i, part) in lines.iter().enumerate() {
            if i > 0 {
                expr.push_str(" & return & ");
            }
            expr.push_str(part);
        }
        expr.push(')');
        expr
    } else {
        format!("\"{}\"", util_escape(input))
    }
}

/// Run `/usr/bin/osascript` with the given `-e` expressions and a timeout (seconds).
///
/// - `e_lines`: each item becomes a `-e` argument for osascript (they should be full AppleScript expressions,
///   for example: `return \"hello\"` or `tell application \"iTerm2\" to ...`).
/// - `timeout_secs`: number of seconds to wait before killing the process.
///
/// Returns the stdout (UTF-8) with normalized line endings (LF) on success, or an error on failure/timeout.
pub fn osascript_with_timeout(e_lines: &[&str], timeout_secs: u64) -> Result<String> {
    let mut cmd = Command::new("/usr/bin/osascript");
    for line in e_lines {
        cmd.arg("-e").arg(line);
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .with_context(|| format!("failed to spawn /usr/bin/osascript with args {:?}", e_lines))?;

    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);

    loop {
        match child.try_wait() {
            Ok(Some(_status)) => {
                // Process finished; collect output
                let output = child
                    .wait_with_output()
                    .context("failed to collect osascript output")?;
                // Normalize line endings: convert CRLF and CR -> LF for predictable comparisons.
                let mut out_str = String::from_utf8_lossy(&output.stdout).to_string();
                out_str = out_str.replace("\r\n", "\n").replace('\r', "\n");
                return Ok(out_str);
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    // Timeout exceeded
                    let _ = child.kill();
                    return Err(anyhow!(
                        "osascript timed out after {} seconds",
                        timeout_secs
                    ));
                }
                // Sleep briefly and poll again
                sleep(Duration::from_millis(50));
            }
            Err(e) => {
                return Err(anyhow!("error while waiting for osascript process: {}", e));
            }
        }
    }
}

/// Trait abstraction for running osascript-like commands.
///
/// Allows production code to use a real system runner while unit tests supply a mock runner.
pub trait OsascriptRunner: Send + Sync {
    /// Run the given apple-script `e_lines` with a timeout and return stdout.
    fn run(&self, e_lines: &[&str], timeout_secs: u64) -> Result<String>;
}

/// System runner that executes the real `/usr/bin/osascript`.
#[derive(Debug, Default)]
pub struct SystemOsascriptRunner {}

impl SystemOsascriptRunner {
    pub fn new() -> Self {
        SystemOsascriptRunner {}
    }
}

impl OsascriptRunner for SystemOsascriptRunner {
    fn run(&self, e_lines: &[&str], timeout_secs: u64) -> Result<String> {
        osascript_with_timeout(e_lines, timeout_secs)
    }
}

/// A simple programmable in-memory mock `OsascriptRunner`.
///
/// Behavior:
/// - Provide a queue of responses (Vec<String>) that get returned in order for each `run` call.
/// - If the queue is empty, `run` returns an error.
/// - Useful for unit tests and CI where calling the real `osascript` is undesirable.
#[derive(Debug, Clone)]
pub struct MockOsascriptRunner {
    inner: Arc<Mutex<VecDeque<String>>>,
}

impl MockOsascriptRunner {
    /// Create a new mock runner seeded with the provided responses.
    pub fn new(responses: Vec<String>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(responses.into_iter().collect())),
        }
    }

    /// Create an empty mock runner.
    pub fn empty() -> Self {
        Self {
            inner: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Push an additional response to the back of the queue.
    pub fn push_response(&self, resp: String) {
        let mut q = self.inner.lock().unwrap();
        q.push_back(resp);
    }
}

impl OsascriptRunner for MockOsascriptRunner {
    fn run(&self, _e_lines: &[&str], _timeout_secs: u64) -> Result<String> {
        let mut q = self.inner.lock().unwrap();
        match q.pop_front() {
            Some(resp) => Ok(resp),
            None => Err(anyhow!(
                "MockOsascriptRunner: no more responses available (called with {:?})",
                _e_lines
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_single_line_quotes_and_backslashes() {
        let input = r#"This is "quoted" and a backslash: \"#;
        let out = escape(input);
        assert!(out.starts_with('"') && out.ends_with('"'));
        // Should contain escaped double quotes and escaped backslashes
        assert!(
            out.contains("\\\""),
            "escaped must contain \\\"; got: {}",
            out
        );
        assert!(
            out.contains("\\\\"),
            "escaped must contain \\\\; got: {}",
            out
        );
        assert!(out.contains("This is "));
        assert!(out.contains("quoted"));
    }

    #[test]
    fn escape_with_tabs_and_unicode_and_large_input() {
        let input = "tab\tunicode: \u{1F600}";
        let out = escape(input);
        assert!(out.contains("\t"));
        assert!(out.contains("unicode:"));
        assert!(out.contains("😀"));
        let large = "x".repeat(20_000);
        let out_large = escape(&large);
        assert!(out_large.len() >= large.len());
    }

    #[test]
    fn escape_multiline_composition_and_escaping_edgecases() {
        let input = "line1\nline\"2\\\n第三行\tend";
        let out = escape(input);
        assert!(
            out.starts_with('(') && out.ends_with(')'),
            "multiline should be expression: {}",
            out
        );
        assert!(
            out.contains(" & return & "),
            "expected concatenation with return, got: {}",
            out
        );
        assert!(
            out.contains("\\\""),
            "expected escaped double quote in multiline: {}",
            out
        );
        assert!(
            out.contains("\\\\"),
            "expected escaped backslash sequence in multiline: {}",
            out
        );
        assert!(out.contains("第三行"));
        assert!(out.contains("\t"));
    }

    #[test]
    fn mock_runner_consumes_responses_in_order() {
        let responses = vec!["a".to_string(), "b".to_string()];
        let mock = MockOsascriptRunner::new(responses);
        let r1 = mock.run(&["ignore"], 1).expect("first response");
        assert_eq!(r1, "a");
        let r2 = mock.run(&["ignore"], 1).expect("second response");
        assert_eq!(r2, "b");
        let err = mock.run(&["ignore"], 1).unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("no more responses"));
    }

    // System runner smoke tests that actually call osascript — only run on macOS.
    #[cfg(target_os = "macos")]
    mod macos_integration {
        use super::*;
        use crate::mcp::iterm::applescript::escape;

        #[test]
        fn system_runner_roundtrip_single_line() {
            let input = r#"Hello "世界" \ path \ test"#;
            let expr = escape(input);
            let script = format!("return {}", expr);
            let runner = SystemOsascriptRunner::new();
            let out = runner
                .run(&[script.as_str()], 5)
                .expect("osascript returned ok");
            let out_trimmed = out
                .trim_end_matches(|c: char| c == '\n' || c == '\r')
                .to_string();
            assert_eq!(out_trimmed, input);
        }

        #[test]
        fn system_runner_roundtrip_multiline() {
            let input = "line1\nline\"2\\\n第三行\tend";
            let expr = escape(input);
            let script = format!("return {}", expr);
            let runner = SystemOsascriptRunner::new();
            let out = runner
                .run(&[script.as_str()], 5)
                .expect("osascript returned ok");
            let out_normalized = out.replace("\r\n", "\n").replace('\r', "\n");
            let out_trimmed = out_normalized.trim_end_matches('\n').to_string();
            assert_eq!(out_trimmed, input);
        }

        #[test]
        fn system_runner_timeout_behaviour() {
            // Use `delay` to force a sleep and set tiny timeout.
            let runner = SystemOsascriptRunner::new();
            let res = runner.run(&["delay 3"], 1);
            assert!(
                res.is_err(),
                "expected timeout error, got ok: {:?}",
                res.ok()
            );
        }
    }
}

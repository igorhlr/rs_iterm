#![cfg(target_os = "macos")]

/*!
macOS-only integration tests for the AppleScript wrapper.

These tests exercise `escape_applescript` and `osascript_with_timeout`
against the system `osascript` binary. They are intentionally simple and
safe (they do not require iTerm2 to be present) — they use `return`
and `delay` AppleScript primitives so they can run on any macOS system.

Note: this file is an integration-style test module. To run it you must
include it in the test module tree (e.g. via `mod` in the tests/mod.rs)
or run the crate tests normally if your test harness already picks it up.
*/

use crate::mcp::iterm::{escape_applescript, osascript_with_timeout};

/// Helper: call `osascript` to `return <expr>` and collect output (trimmed).
fn osascript_return(expr: &str) -> anyhow::Result<String> {
    // Build a one-line AppleScript that returns the expression
    let script = format!("return {}", expr);
    let out = osascript_with_timeout(&[script.as_str()], 5)?;
    // osascript typically appends a newline; trim end for robust comparison
    Ok(out
        .trim_end_matches(|c: char| c == '\n' || c == '\r')
        .to_string())
}

#[test]
fn roundtrip_single_line_escape_and_return() {
    // Single-line content with quotes, backslashes and unicode
    let input = r#"Hello "世界" \ path \ test"#;
    // Escape for AppleScript
    let expr = escape_applescript(input);
    // The expression should be quoted (single-line)
    assert!(
        expr.starts_with('"') && expr.ends_with('"'),
        "escaped expression not quoted: {}",
        expr
    );

    // Call osascript to return the escaped expression and compare with original
    let returned = osascript_return(&expr)
        .expect("osascript should successfully return single-line expression");

    assert_eq!(
        returned, input,
        "roundtrip single-line value mismatch: expected {:?}, got {:?}",
        input, returned
    );
}

#[test]
fn roundtrip_multiline_escape_and_return() {
    // Multi-line input with quotes, backslashes, tabs and unicode
    let input = "line1\nline\"2\\\n第三行\tend";
    let expr = escape_applescript(input);

    // For multiline we expect a parenthesized concatenation expression
    assert!(
        expr.starts_with('(') && expr.ends_with(')'),
        "multiline escaped expression should be parenthesized: {}",
        expr
    );
    assert!(
        expr.contains(" & return & "),
        "multiline escaped expression should use ` & return & ` concatenation: {}",
        expr
    );

    // Ask osascript to return the composed expression
    let returned =
        osascript_return(&expr).expect("osascript should successfully return multiline expression");

    // The returned string should match the original exactly (preserving newlines and tabs)
    assert_eq!(
        returned, input,
        "roundtrip multiline value mismatch: expected {:?}, got {:?}",
        input, returned
    );
}

#[test]
fn osascript_with_timeout_times_out() {
    // Use AppleScript `delay` to sleep for longer than our timeout
    let long_sleep_script = "delay 3"; // sleep 3 seconds
                                       // Call with a tiny timeout to force the timeout branch
    let res = osascript_with_timeout(&[long_sleep_script], 1);

    assert!(
        res.is_err(),
        "expected osascript_with_timeout to return Err on timeout, got Ok: {:?}",
        res.ok()
    );

    // Validate error message mentions timeout seconds (implementation detail)
    if let Err(e) = res {
        let msg = format!("{}", e);
        assert!(
            msg.contains("timed out") || msg.contains("timed out after"),
            "expected timeout-related error, got: {}",
            msg
        );
    }
}

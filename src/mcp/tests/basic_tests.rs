//! Basic unit tests for utilities and tool registration.
//!
//! This test file verifies:
//! - `escape_applescript_string` behavior
//! - `letter_to_control_char` mapping and error cases
//! - `register_tools` returns the expected tools with parameter schemas
//!
//! These are lightweight checks to ensure the initial code shape is correct and
//! the tool registration exposes the expected tool names and parameter keys.

use serde_json::Value;
use std::collections::HashMap;

use crate::mcp::tools::register_tools;
use crate::mcp::utilities::{escape_applescript_string, letter_to_control_char};

#[test]
fn test_escape_applescript_string_basic() {
    let input = r#"This is a "quote" and a backslash: \ and newline
and more text"#;
    let escaped = escape_applescript_string(input);

    // Ensure backslashes and double-quotes are escaped
    assert!(
        escaped.contains(r#"\""#),
        "escaped should contain escaped double-quote: got '{}'",
        escaped
    );
    // Ensure at least one backslash was escaped (becomes "\\")
    assert!(
        escaped.contains("\\\\"),
        "escaped should contain escaped backslash sequence (at least one backslash escaped): got '{}'",
        escaped
    );

    // Ensure original content's substrings are still present (but escaped where needed)
    assert!(escaped.contains("This is a "));
    assert!(escaped.contains("quote"));
    assert!(escaped.contains("newline"));
}

#[test]
fn test_letter_to_control_char_valid_and_special() {
    // 'A' -> 1, 'B' -> 2, 'C' -> 3
    let a = letter_to_control_char("A").expect("A should map");
    let b = letter_to_control_char("b").expect("b should map (case-insensitive)");
    let c = letter_to_control_char("C").expect("C should map");

    assert_eq!(a, 1u8);
    assert_eq!(b, 2u8);
    assert_eq!(c, 3u8);

    // Special mapping: ']' -> 29 (as defined in utilities)
    let special = letter_to_control_char("]").expect("] should map to special code");
    assert_eq!(special, 29u8);

    // Another special: '@' -> 0 (NUL)
    let nul = letter_to_control_char("@").expect("@ should map to NUL (0)");
    assert_eq!(nul, 0u8);
}

#[test]
fn test_letter_to_control_char_invalid() {
    // Empty string is invalid
    assert!(
        letter_to_control_char("").is_err(),
        "empty input should be an error"
    );

    // Multi-character string should be invalid
    assert!(
        letter_to_control_char("AB").is_err(),
        "multi-character input should be an error"
    );

    // Non-mapped character (e.g., '1') should error
    assert!(
        letter_to_control_char("1").is_err(),
        "numeric character should not be valid"
    );
}

#[test]
fn test_register_tools_contains_expected_tools_and_schemas() {
    let tools = register_tools();

    // Expected tool names from the current implementation
    let expected = vec![
        "iterm-mcp:write_to_terminal",
        "iterm-mcp:read_terminal_output",
        "iterm-mcp:send_control_character",
    ];

    for name in expected.iter() {
        assert!(
            tools.contains_key(*name),
            "registered tools should contain '{}'; found keys: {:?}",
            name,
            tools.keys().collect::<Vec<&String>>()
        );

        // Inspect the definition and ensure parameter schema has expected structure
        let (def, _handler) = tools.get(*name).expect("tool must exist");
        // parameters is a HashMap<String, serde_json::Value>
        // We expect at least a "properties" entry which contains the parameter names.
        let properties_val = def.parameters.get("properties").expect(&format!(
            "Tool '{}' should have 'properties' in its parameters",
            name
        ));

        // Ensure properties is an object and contains the expected parameter key per tool
        match properties_val {
            Value::Object(map) => {
                let required_key = match *name {
                    "iterm-mcp:write_to_terminal" => "command",
                    "iterm-mcp:read_terminal_output" => "linesOfOutput",
                    "iterm-mcp:send_control_character" => "letter",
                    _ => panic!("unexpected tool name"),
                };

                assert!(
                    map.contains_key(required_key),
                    "Tool '{}' properties should include key '{}', got keys: {:?}",
                    name,
                    required_key,
                    map.keys().collect::<Vec<&String>>()
                );
            }
            other => panic!(
                "Tool '{}' parameters.properties must be an object, got: {:?}",
                name, other
            ),
        }
    }
}

// Extra sanity test: ensure registered tool count is at least 3
#[test]
fn test_register_tools_minimum_count() {
    let tools = register_tools();
    assert!(
        tools.len() >= 3,
        "Expected at least 3 tools to be registered, got {}",
        tools.len()
    );
}

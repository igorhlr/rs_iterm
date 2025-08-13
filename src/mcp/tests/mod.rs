//! Test module entry for `mcp` tests.
//!
//! This file registers the individual test files under `src/mcp/tests/`
//! so they are compiled and run with `cargo test` when building the crate.
#![cfg(test)]

mod basic_tests;

#[cfg(target_os = "macos")]
mod integration_applescript;

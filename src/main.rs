use std::env;
use std::process;

use anyhow::{Context, Result};
use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod mcp;

/// iTerm MCP server implementation in Rust
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Set the log level (trace, debug, info, warn, error)
    #[clap(short, long, default_value = "info")]
    log_level: String,

    /// Port to listen on
    #[clap(short, long, default_value = "3000")]
    port: u16,

    /// Address to bind to
    #[clap(long, default_value = "127.0.0.1")]
    address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Setup logging
    let log_level = match args.log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set global logging subscriber")?;
    
    info!("Starting iTerm MCP server on {}:{}", args.address, args.port);
    
    // Check if we're running on macOS
    if env::consts::OS != "macos" {
        eprintln!("Error: This application only runs on macOS.");
        process::exit(1);
    }
    
    // Initialize and start the MCP server
    let server = mcp::server::start_server(args.address, args.port).await?;
    
    // Wait for the server to finish
    server.await?;
    
    info!("iTerm MCP server has stopped");
    Ok(())
}

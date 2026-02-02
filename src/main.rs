use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;
use url_default::engine::Engine;

#[derive(Parser)]
#[command(name = "url-default")]
#[command(about = "Routes URLs to specific browsers based on config")]
struct Cli {
    /// The URL to open
    url: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let url = &cli.url;

    // Load engine
    let engine = Engine::new("config.json")?;
    let target = engine.match_url(url);
    
    println!("Routing '{}' to '{}'", url, target);

    let status = if target == "SystemDefault" {
        Command::new("open")
            .arg(url)
            .status()
            .context("Failed to open default browser")?
    } else {
        Command::new("open")
            .arg("-a")
            .arg(&target)
            .arg(url)
            .status()
            .with_context(|| format!("Failed to open application: {}", target))?
    };

    if !status.success() {
        eprintln!("Command failed to execute successfully");
        std::process::exit(1);
    }

    Ok(())
}

use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;
use url_default::engine::Engine;
use cacao::appkit::{App, AppDelegate};
use cacao::url::Url;

// Reuse the CLAP struct for manual CLI usage, though Cacao will take over main loop.
#[derive(Parser)]
#[command(name = "url-default")]
struct Cli {
    #[arg(required = false)]
    url: Option<String>,
}

struct RouterApp;

impl AppDelegate for RouterApp {
    fn open_urls(&self, urls: Vec<Url>) {
        // Log that we received URLs
        std::fs::write("/tmp/url-router.cacao.log", format!("Received URLs: {:?}\n", urls)).ok();

        for url in urls {
             // Convert cacao::url::Url to string
             let url_str = url.to_string();
             let _ = handle_url_logic(&url_str);
        }
        // After handling, we should probably exit?
        // But maybe there are multiple URLs?
        // For a router, usually we handle one and exit.
        std::process::exit(0);
    }
    
    fn did_finish_launching(&self) {
        // Check if we have CLI args that Cacao didn't handle (though Cacao hijacks main).
        // Actually, if we launch locally with ./url-default <URL>, Cacao might not pick it up as open_url?
        // But for the default browser use case, `open_urls` is what matters.
        
        // Let's check CLI args manually just in case
        let args: Vec<String> = std::env::args().collect();
        // Skip argv[0]
        if args.len() > 1 {
             // Basic check, might collide with standard cocoa args
             let arg1 = &args[1];
             if !arg1.starts_with("-") {
                 let _ = handle_url_logic(arg1);
                 std::process::exit(0);
             }
        }
    }
}

fn handle_url_logic(url: &str) -> Result<()> {
    // Load engine
    let engine = Engine::new("config.json")?;
    let target = engine.match_url(url);
    
    println!("Routing '{}' to '{}'", url, target);
    std::fs::write("/tmp/url-router.last.log", format!("Handling URL: {}\nTarget: {}\n", url, target)).ok();

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
        return Err(anyhow::anyhow!("Command failed"));
    }
    Ok(())
}

fn main() {
    // Log startup
    std::fs::write("/tmp/url-router.start.log", format!("Started with args: {:?}\n", std::env::args().collect::<Vec<_>>())).ok();
    
    App::new("com.example.url-default", RouterApp).run();
}

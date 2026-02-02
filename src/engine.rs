use crate::config::Config;
use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

pub struct Engine {
    config: Config,
    compiled_rules: Vec<(Regex, String)>,
}

impl Engine {
    pub fn new(config_name: &str) -> Result<Self> {
        let paths_to_check = vec![
            // 1. Current working directory
            std::env::current_dir()?.join(config_name),
            // 2. Executable directory (for .app bundle)
            std::env::current_exe()?
                .parent()
                .context("Failed to get executable directory")?
                .join(config_name),
            // 3. Executable directory/../Resources (standard macOS app structure)
            std::env::current_exe()?
                .parent()
                .context("Failed to get executable directory")?
                .parent()
                .unwrap_or(Path::new("/"))
                .join("Resources")
                .join(config_name),
        ];

        for path in paths_to_check {
            if path.exists() {
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read config file from {:?}", path))?;
                let config: Config = serde_json::from_str(&content)
                    .context("Failed to parse config file")?;
                return Self::from_config(config);
            }
        }

        anyhow::bail!("Config file '{}' not found in search paths", config_name);
    }

    pub fn from_config(config: Config) -> Result<Self> {
        let mut compiled_rules = Vec::new();
        for rule in &config.rules {
            let regex = Regex::new(&rule.pattern)
                .with_context(|| format!("Invalid regex pattern: {}", rule.pattern))?;
            compiled_rules.push((regex, rule.target.clone()));
        }

        Ok(Self {
            config,
            compiled_rules,
        })
    }

    pub fn match_url(&self, url: &str) -> String {
        for (regex, target) in &self.compiled_rules {
            if regex.is_match(url) {
                return target.clone();
            }
        }
        self.config.default_browser.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_logic() {
        let config = Config {
            default_browser: "Safari".to_string(),
            rules: vec![
                Rule {
                    comment: None,
                    pattern: r"^https://zoom\.us/".to_string(),
                    target: "Zoom".to_string(),
                },
                Rule {
                    comment: None,
                    pattern: "google".to_string(),
                    target: "Chrome".to_string(),
                },
            ],
        };

        let engine = Engine::from_config(config).unwrap();

        assert_eq!(engine.match_url("https://zoom.us/j/123"), "Zoom");
        assert_eq!(engine.match_url("https://google.com"), "Chrome");
        assert_eq!(engine.match_url("https://example.com"), "Safari");
    }
}

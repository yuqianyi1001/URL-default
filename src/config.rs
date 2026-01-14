use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_browser: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub comment: Option<String>,
    pub pattern: String,
    pub target: String,
}

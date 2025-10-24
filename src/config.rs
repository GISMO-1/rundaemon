use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub services: Vec<Service>,
    #[serde(default)]
    pub log_level: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default = "default_restart")]
    pub restart: Restart,
    #[serde(default)]
    pub health: Option<Health>,
    #[serde(default)]
    pub env: Vec<EnvVar>,
    #[serde(default)]
    pub working_dir: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Restart { Always, OnFailure, Never }

fn default_restart() -> Restart { Restart::OnFailure }

#[derive(Debug, Deserialize, Clone)]
pub struct EnvVar { pub key: String, pub value: String }

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Health {
    Http { url: String, #[serde(default = "default_timeout_ms")] timeout_ms: u64, #[serde(default = "default_interval_ms")] interval_ms: u64 },
    Cmd  { cmd: String, #[serde(default)] args: Vec<String>, #[serde(default = "default_interval_ms")] interval_ms: u64 },
}

fn default_timeout_ms() -> u64 { 2000 }
fn default_interval_ms() -> u64 { 3000 }

impl Config {
    pub fn from_path(p: &str) -> anyhow::Result<Self> {
        let s = std::fs::read_to_string(p)?;
        Ok(serde_yaml::from_str(&s)?)
    }
}

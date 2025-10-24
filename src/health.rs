use crate::config::Health;
use anyhow::Result;
use tokio::{process::Command, time::{sleep, Duration}};
use tracing::{info, warn};

pub async fn monitor(name: String, h: Health) {
    match h {
        Health::Http { url, timeout_ms, interval_ms } => {
            let client = reqwest::Client::new();
            loop {
                let res = client.get(&url).timeout(Duration::from_millis(timeout_ms)).send().await;
                match res {
                    Ok(r) if r.status().is_success() => info!(%name, "health ok {}", url),
                    Ok(r) => warn!(%name, code=?r.status(), "health bad {}", url),
                    Err(e) => warn!(%name, error=?e, "health error {}", url),
                }
                sleep(Duration::from_millis(interval_ms)).await;
            }
        }
        Health::Cmd { cmd, args, interval_ms } => {
            loop {
                let status = Command::new(&cmd).args(&args).status().await;
                match status {
                    Ok(s) if s.success() => info!(%name, "health ok cmd={}", cmd),
                    Ok(s) => warn!(%name, code=?s.code(), "health bad cmd={}", cmd),
                    Err(e) => warn!(%name, error=?e, "health error cmd={}", cmd),
                }
                sleep(Duration::from_millis(interval_ms)).await;
            }
        }
    }
}

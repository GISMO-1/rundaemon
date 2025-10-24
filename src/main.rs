mod config;
mod supervisor;
mod health;
mod logging;

use anyhow::Result;
use tokio::task;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| "examples/sample.yml".to_string());
    let cfg = config::Config::from_path(&path)?;
    logging::init(cfg.log_level.as_deref().unwrap_or("info"));

    // spawn health monitors
    for svc in &cfg.services {
        if let Some(h) = svc.health.clone() {
            let name = svc.name.clone();
            task::spawn(health::monitor(name, h));
        }
    }

    info!("rundaemon starting");
    supervisor::run(cfg.services).await?;
    info!("rundaemon stopped");
    Ok(())
}

use tracing_subscriber::{fmt, EnvFilter};

pub fn init(default_level: &str) {
    let env = std::env::var("RUST_LOG").unwrap_or_else(|_| default_level.to_string());
    tracing_subscriber::registry()
        .with(EnvFilter::new(env))
        .with(fmt::layer().with_target(false))
        .init();
}

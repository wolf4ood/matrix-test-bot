use anyhow::Result;
use std::{convert::TryInto, path::PathBuf};

pub mod bot;
pub mod cfg;
pub mod handler;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config: cfg::BotConfig = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::default().join("config.toml"))
        .try_into()?;

    bot::run(config).await?;

    Ok(())
}

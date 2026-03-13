//! Synth AI Arbitrage Bot
//! 
//! Monitors Synth AI forecasts (Bittensor SN50) and compares them with Polymarket
//! implied odds to detect and trade on mispricings.

mod config;
mod services;
mod utils;

use anyhow::Result;
use config::Env;
use services::synth_arbitrage::start_synth_arbitrage;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::load();
    
    println!("{}", "\n╔════════════════════════════════════════════════════════════════╗".cyan().bold());
    println!("{}", "║     Synth AI Arbitrage Bot - Bittensor SN50 Integration      ║".cyan().bold());
    println!("{}", "╚════════════════════════════════════════════════════════════════╝\n".cyan().bold());

    println!("{}", "Configuration:".yellow().bold());
    println!("  Synth API URL: {}", std::env::var("SYNTH_API_URL").unwrap_or_else(|_| "https://api.synth.ai".to_string()));
    println!("  Min Edge: {}%", std::env::var("SYNTH_MIN_EDGE_PERCENT").unwrap_or_else(|_| "10.0".to_string()));
    println!("  Base Trade Size: ${}", std::env::var("SYNTH_BASE_TRADE_SIZE_USD").unwrap_or_else(|_| "50.0".to_string()));
    println!("  Check Interval: {}s", std::env::var("SYNTH_CHECK_INTERVAL_SECS").unwrap_or_else(|_| "60".to_string()));
    println!();

    if env.preview_mode {
        println!("{}", "🔍 PREVIEW MODE ENABLED - No actual trades will be executed".yellow().bold());
        println!();
    }

    start_synth_arbitrage(&env).await?;

    Ok(())
}

# Synth AI Integration Guide

## Overview

The Synth AI integration enables the bot to query probabilistic forecasts from Bittensor SN50 (Synth) and compare them with Polymarket implied odds to detect trading edges of 5-15%+.

## Features

- 🤖 **AI-Powered Edge Detection**: Uses Synth's probabilistic forecasts to identify mispricings
- 📊 **Multi-Market Support**: Monitors BTC, ETH, SOL markets
- ⏱️ **Multiple Timeframes**: Supports 15-minute, hourly, and daily markets
- 🎯 **Configurable Thresholds**: Set minimum edge percentage and trade sizes
- 🔍 **Preview Mode**: Test strategies without executing trades

## Setup

### 1. Environment Variables

Add these to your `.env` file:

```bash
# Synth API Configuration
SYNTH_API_URL=https://api.synth.ai          # Synth API endpoint
SYNTH_API_KEY=your_api_key_here             # Optional: API key if required

# Trading Configuration
SYNTH_MIN_EDGE_PERCENT=10.0                 # Minimum edge % to trigger trade (default: 10%)
SYNTH_BASE_TRADE_SIZE_USD=50.0              # Base trade size in USD (default: $50)
SYNTH_CHECK_INTERVAL_SECS=60                # How often to check markets (default: 60s)

# Preview Mode (set to false for live trading)
PREVIEW_MODE=true
```

### 2. Running the Bot

```bash
# Run Synth arbitrage bot
cargo run --bin synth_arbitrage

# Or build and run
cargo build --release --bin synth_arbitrage
./target/release/synth_arbitrage
```

## How It Works

1. **Forecast Query**: Bot queries Synth API for probabilistic forecasts on BTC/ETH/SOL markets
2. **Market Discovery**: Finds active Polymarket markets matching the symbol/timeframe
3. **Price Comparison**: Compares Synth probabilities with Polymarket implied odds
4. **Edge Detection**: Calculates mispricing percentage
5. **Trade Execution**: If edge >= threshold, executes trade (if not in preview mode)

## Example Output

```
╔════════════════════════════════════════════════════════════════╗
║     Synth AI Arbitrage Service - Starting Monitor            ║
╚════════════════════════════════════════════════════════════════╝

✓ Minimum edge threshold: 10%
✓ Base trade size: $50.00
✓ Check interval: 60 seconds

🤖 [SYNTH AI] Edge Detected - BTC 15m
   Synth Forecast: UP=65.00% DOWN=35.00% (Confidence: 85.0%)
   Polymarket Implied: UP=50.00% DOWN=50.00%
   Edge: 15.00% in UP direction
   Recommended trade size: $63.75
```

## API Integration

The Synth client expects the API to return forecasts in this format:

```json
{
  "symbol": "BTC",
  "timeframe": "15m",
  "probability_up": 0.65,
  "probability_down": 0.35,
  "confidence": 0.85,
  "timestamp": 1234567890,
  "source": "bittensor_sn50"
}
```

## Trading Logic

- **Edge Calculation**: `edge = synth_probability - polymarket_implied_probability`
- **Trade Size**: Scaled based on edge magnitude and confidence
  - Formula: `trade_size = base_size * (edge/10) * confidence` (capped at 2x)
- **Direction**: Trades in the direction with the larger edge

## Notes

- The Synth API endpoint and authentication may need to be configured based on actual Synth/Bittensor API documentation
- Currently uses placeholder Polymarket prices - full integration requires orderbook price fetching
- Trade execution is not yet fully implemented - requires CLOB client integration
- Preview mode is enabled by default for safety

## Future Enhancements

- [ ] Full orderbook price integration
- [ ] CLOB client integration for trade execution
- [ ] Risk management and position sizing
- [ ] Historical backtesting
- [ ] Multi-timeframe analysis
- [ ] Range market support

# 🤖 AI Trading Bot | Polymarket Trading Bots Suite

> **Professional AI trading bot suite for Polymarket prediction markets. Advanced AI-powered trading bots featuring Synth AI integration, automated arbitrage detection, copy trading, and market making. Built with Rust for high-performance algorithmic trading.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-ISC-yellow.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Production%20Ready-green.svg)]()

---

## 🎯 Overview

**AI trading bots** are intelligent automated software programs that execute trades on Polymarket, a decentralized prediction market platform. This professional AI trading bot suite includes advanced algorithms and machine learning capabilities:

- **🤖 Synth AI Arbitrage Bot** - Uses Bittensor SN50 AI forecasts to detect 5-15%+ mispricings
- **📈 Copy Trading Bot** - Automatically copies successful traders in real-time
- **⚡ Latency Arbitrage Bot** - Exploits price lags between exchanges and Polymarket
- **📊 Market Maker Bot** - Provides liquidity and earns maker rebates

### Why Use AI Trading Bots?

- **24/7 Operation**: Trade while you sleep
- **Speed**: Execute trades in milliseconds with AI-powered decision making
- **Emotion-Free**: Remove human bias and fear with algorithmic trading
- **Consistency**: Follow strategies without deviation
- **Scalability**: Handle multiple markets simultaneously
- **AI Edge Detection**: Leverage machine learning to find profitable opportunities

---

## ✨ Key Features

### 🤖 Synth AI Edge Integration

AI trading bot queries Bittensor SN50 (Synth) probabilistic forecasts via SDK/API to detect and auto-trade 5-15%+ mispricings vs. Polymarket implied odds on BTC/ETH/SOL hourly/15-min/daily Up-Down markets.

**How it works:**
1. Bot queries Synth AI for probabilistic forecasts
2. Algorithm compares forecasts with Polymarket implied odds
3. Detects mispricings when edge ≥ threshold (default 10%)
4. Automatically executes trades when profitable

### 👥 Copy Trading Bot

**Real-time copy trading** from successful Polymarket traders:

- **Multiple Strategies**: PERCENTAGE, FIXED, or ADAPTIVE position sizing
- **Multi-Trader Support**: Copy several traders simultaneously
- **Take Profit & Stop Loss**: Automated profit locking and loss limiting
- **Trade Aggregation**: Combines small trades into larger orders
- **MongoDB Persistence**: Complete trade history and position tracking
- **Preview Mode**: Test strategies without executing trades

### ⚡ Latency Arbitrage Bot

**Detect and exploit arbitrage opportunities** in 15-minute crypto markets:

- Real-time WebSocket monitoring
- Automatic arbitrage detection (`UP_ASK + DOWN_ASK < 1.0`)
- Supports BTC, ETH, SOL, XRP markets
- Interactive terminal UI
- Duplicate opportunity prevention

### 📊 Market Maker Bot

**Automated market making** for Polymarket CLOB:

- Two strategies: AMM (Automated Market Maker) and Bands
- Configurable sync intervals
- Prometheus metrics integration
- Graceful shutdown handling

---

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://www.rust-lang.org/tools/install)
- **Polymarket Account** with wallet and USDC balance
- **Polygon RPC URL** - Get from [Alchemy](https://www.alchemy.com/) or [Infura](https://www.infura.io/)
- **MongoDB** (for copy trading bot) - [MongoDB Atlas](https://www.mongodb.com/cloud/atlas) (free tier available)

### Installation

```bash
# Clone the repository
git clone https://github.com/Willis404/Professional-Polymarket-Trading-Bots-Suite-AI-Powered-Arbitrage-Copy-Trading.git
cd Professional-Polymarket-Trading-Bots-Suite-AI-Powered-Arbitrage-Copy-Trading

# Build all bots
cargo build --release
```

### Configuration

Create a `.env` file in the bot directory:

```env
# Wallet Configuration
PRIVATE_KEY=your_wallet_private_key_here
PROXY_WALLET=your_proxy_wallet_address_here

# API Endpoints
CLOB_HTTP_URL=https://clob.polymarket.com
CLOB_WS_URL=wss://ws-subscriptions-clob.polymarket.com/ws
RPC_URL=https://polygon-rpc.com
MONGO_URI=mongodb://localhost:27017/polymarket_bot

# Trading Configuration
COPY_STRATEGY=PERCENTAGE
COPY_SIZE=10.0
MAX_ORDER_SIZE_USD=100.0
MIN_ORDER_SIZE_USD=0.01

# Synth AI (Optional)
SYNTH_API_URL=https://api.synth.ai
SYNTH_API_KEY=your_synth_api_key
SYNTH_MIN_EDGE_PERCENT=10.0

# Preview Mode (set to false for live trading)
PREVIEW_MODE=true
```

### Running the Bots

```bash
# Copy Trading Bot
cd automated-copy-trading-system
cargo run --release

# Arbitrage Bot
cd ai-synth-arbitrage-engine
cargo run --release

# Synth AI Arbitrage Bot
cd ai-synth-arbitrage-engine
cargo run --bin synth_arbitrage

# Market Maker Bot
cd liquidity-provider-bot
cargo run --release -- \
  --private-key <your-key> \
  --rpc-url <rpc-url> \
  --clob-api-url https://clob.polymarket.com \
  --condition-id <condition-id> \
  --strategy amm \
  --strategy-config ./config/amm.json
```
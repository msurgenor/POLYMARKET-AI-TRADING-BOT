use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Synth AI forecast response from Bittensor SN50
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthForecast {
    pub symbol: String,           // BTC, ETH, SOL, etc.
    pub timeframe: String,        // "1h", "15m", "1d"
    pub probability_up: f64,       // Synth's probability of price going up (0.0-1.0)
    pub probability_down: f64,    // Synth's probability of price going down (0.0-1.0)
    pub confidence: f64,          // Confidence score (0.0-1.0)
    pub timestamp: i64,           // Unix timestamp
    pub source: String,           // "bittensor_sn50" or "synth"
}

/// Synth API client for querying probabilistic forecasts
pub struct SynthClient {
    api_url: String,
    api_key: Option<String>,
    client: reqwest::Client,
}

impl SynthClient {
    pub fn new(api_url: Option<String>, api_key: Option<String>) -> Self {
        Self {
            api_url: api_url.unwrap_or_else(|| "https://api.synth.ai".to_string()),
            api_key,
            client: reqwest::Client::new(),
        }
    }

    /// Query Synth for probabilistic forecast for a given market
    /// 
    /// # Arguments
    /// * `symbol` - Cryptocurrency symbol (BTC, ETH, SOL, etc.)
    /// * `timeframe` - Timeframe ("1h", "15m", "1d")
    /// * `market_type` - "up_down" or "range"
    pub async fn get_forecast(
        &self,
        symbol: &str,
        timeframe: &str,
        market_type: &str,
    ) -> Result<SynthForecast> {
        let url = format!(
            "{}/v1/forecast?symbol={}&timeframe={}&market_type={}",
            self.api_url.trim_end_matches('/'),
            symbol,
            timeframe,
            market_type
        );

        let mut request = self.client.get(&url);

        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Synth API error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            ));
        }

        let forecast: SynthForecast = response.json().await?;
        Ok(forecast)
    }

    /// Batch query multiple forecasts
    pub async fn get_forecasts(
        &self,
        requests: Vec<(String, String, String)>, // (symbol, timeframe, market_type)
    ) -> Result<HashMap<String, SynthForecast>> {
        let mut forecasts = HashMap::new();
        
        for (symbol, timeframe, market_type) in requests {
            let key = format!("{}_{}_{}", symbol, timeframe, market_type);
            match self.get_forecast(&symbol, &timeframe, &market_type).await {
                Ok(forecast) => {
                    forecasts.insert(key, forecast);
                }
                Err(e) => {
                    log::warn!("Failed to get forecast for {} {} {}: {}", symbol, timeframe, market_type, e);
                }
            }
        }

        Ok(forecasts)
    }
}

/// Compare Synth forecast with Polymarket implied odds to detect mispricings
#[derive(Debug, Clone)]
pub struct MispricingDetection {
    pub symbol: String,
    pub timeframe: String,
    pub synth_probability_up: f64,
    pub polymarket_implied_up: f64,
    pub edge_percent: f64,
    pub direction: String, // "UP" or "DOWN"
    pub confidence: f64,
    pub timestamp: i64,
}

impl MispricingDetection {
    /// Detect mispricing between Synth forecast and Polymarket odds
    /// 
    /// Returns Some(MispricingDetection) if edge >= min_edge_percent, None otherwise
    pub fn detect(
        synth_forecast: &SynthForecast,
        polymarket_up_price: f64,
        polymarket_down_price: f64,
        min_edge_percent: f64,
    ) -> Option<Self> {
        // Polymarket implied probabilities (prices represent probabilities)
        let polymarket_implied_up = polymarket_up_price;
        let polymarket_implied_down = polymarket_down_price;

        // Synth probabilities
        let synth_up = synth_forecast.probability_up;
        let synth_down = synth_forecast.probability_down;

        // Calculate edges in both directions
        let up_edge = synth_up - polymarket_implied_up;
        let down_edge = synth_down - polymarket_implied_down;

        // Determine which direction has better edge
        let (edge_percent, direction, synth_prob) = if up_edge.abs() > down_edge.abs() {
            (up_edge * 100.0, "UP", synth_up)
        } else {
            (down_edge * 100.0, "DOWN", synth_down)
        };

        // Check if edge meets minimum threshold
        if edge_percent.abs() >= min_edge_percent {
            Some(Self {
                symbol: synth_forecast.symbol.clone(),
                timeframe: synth_forecast.timeframe.clone(),
                synth_probability_up: synth_up,
                polymarket_implied_up,
                edge_percent,
                direction: direction.to_string(),
                confidence: synth_forecast.confidence,
                timestamp: Utc::now().timestamp_millis(),
            })
        } else {
            None
        }
    }

    /// Get recommended trade size based on edge and confidence
    pub fn recommended_trade_size(&self, base_size: f64) -> f64 {
        // Scale trade size based on edge magnitude and confidence
        let edge_multiplier = (self.edge_percent.abs() / 10.0).min(2.0); // Cap at 2x
        let confidence_multiplier = self.confidence;
        base_size * edge_multiplier * confidence_multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mispricing_detection() {
        let synth_forecast = SynthForecast {
            symbol: "BTC".to_string(),
            timeframe: "15m".to_string(),
            probability_up: 0.65,
            probability_down: 0.35,
            confidence: 0.85,
            timestamp: Utc::now().timestamp_millis(),
            source: "bittensor_sn50".to_string(),
        };

        // Polymarket prices imply 50/50 odds
        let detection = MispricingDetection::detect(
            &synth_forecast,
            0.50, // Polymarket UP price
            0.50, // Polymarket DOWN price
            10.0, // Minimum 10% edge
        );

        assert!(detection.is_some());
        let det = detection.unwrap();
        assert_eq!(det.direction, "UP");
        assert!(det.edge_percent >= 10.0);
    }
}

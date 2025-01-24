use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// ID for the user.
    pub id: Uuid,

    /// Name of the user.
    pub name: String,

    /// Profile of the user behavior.
    pub behavior_type: BehaviorType,

    /// Holding period for the user.
    pub holding_period: Option<u64>,

    /// Frequency of trades for the user.
    pub trade_frequency: Option<TradeFrequency>,

    /// Initial balance of tokens or assets allocated to the user.
    pub initial_balance: f64,

    /// Risk tolerance of the user.
    /// This is a value between 0 and 1, where 0 is the lowest risk tolerance and 1 is the highest.
    pub risk_tolerance: f64,
}

/// Profile of the user behavior.
#[derive(Debug, Deserialize, Serialize)]
pub enum BehaviorType {
    /// A user that holds assets.
    #[serde(rename = "holder")]
    Holder,

    /// A user that trades assets.
    #[serde(rename = "trader")]
    Trader,

    /// A user that provides liquidity.
    #[serde(rename = "market_maker")]
    MarketMaker,
}

/// Frequency of trades for the user.
#[derive(Debug, Deserialize, Serialize)]
pub enum TradeFrequency {
    /// Real-time trading.
    #[serde(rename = "real_time")]
    RealTime,

    /// Hourly trading.
    #[serde(rename = "hourly")]
    Hourly,

    /// Daily trading.
    #[serde(rename = "daily")]
    Daily,

    /// Weekly trading.
    #[serde(rename = "weekly")]
    Weekly,

    /// Monthly trading.
    #[serde(rename = "monthly")]
    Monthly,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Simulation {
    /// ID of the simulation.
    pub id: Uuid,

    /// Name of the simulation.
    pub name: String,

    /// Description of the simulation.
    pub description: Option<String>,

    /// Status of the simulation.
    pub status: SimulationStatus,

    /// Duration of the simulation, in days.
    pub duration: u64,

    /// ID of the token used in the simulation.
    pub token_id: Uuid,

    /// IDs of the users participating in the simulation.
    pub user_ids: Vec<Uuid>,

    /// Volatility level.
    /// 0.0 is no volatility, 1.0 is maximum volatility.
    pub market_volatility: f64,

    /// Results of the simulation.
    pub results: SimulationResults,

    /// Date and time the simulation was created.
    pub created_at: DateTime<Utc>,

    /// Date and time the simulation was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Results of a simulation.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulationResults {
    /// Total profit or loss of the simulation.
    pub total_profit_loss: f64,

    /// Total number of trades made in the simulation.
    pub total_trades: u64,

    /// Total number of successful trades made in the simulation.
    pub successful_trades: u64,

    /// Total number of failed trades made in the simulation.
    pub failed_trades: u64,
}

/// Status of a simulation.
#[derive(Debug, Deserialize, Serialize)]
pub enum SimulationStatus {
    /// Simulation is currently running.
    #[serde(rename = "running")]
    Running,

    /// Simulation has completed.
    #[serde(rename = "completed")]
    Completed,

    /// Simulation has been cancelled.
    #[serde(rename = "cancelled")]
    Cancelled,
}

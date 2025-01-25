use std::{collections::HashMap, hash::BuildHasherDefault};

use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use twox_hash::XxHash64;
use uuid::Uuid;

use crate::{SimulationReport, Token, User};

/// Input parameters for a simulation.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulationOptions {
    /// Duration of the simulation, depending on the interval type.
    pub duration: u64,

    /// Number of users in the simulation.
    pub total_users: u64,

    /// Volatility level. 0.0 is no volatility, 1.0 is maximum volatility.
    pub market_volatility: f64,

    /// Interval type for the simulation (daily or weekly).
    pub interval_type: SimulationInterval,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Simulation {
    /// ID of the simulation.
    pub id: Uuid,

    /// Name of the simulation.
    pub name: String,

    /// Token used in the simulation.
    pub token: Token,

    /// Description of the simulation.
    pub description: Option<String>,

    /// Status of the simulation.
    pub status: SimulationStatus,

    /// Input parameters for the simulation.
    pub options: SimulationOptions,

    /// Report of the results for each interval of the simulation.
    pub interval_reports: HashMap<u64, SimulationReport, BuildHasherDefault<XxHash64>>,

    /// Report of the total results of the simulation.
    pub report: SimulationReport,

    /// Date and time the simulation was created.
    pub created_at: DateTime<Utc>,

    /// Date and time the simulation was last updated.
    pub updated_at: DateTime<Utc>,
}

/// Status of a simulation.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum SimulationStatus {
    /// Simulation is currently running.
    #[serde(rename = "running")]
    Running,

    /// Simulation has completed.
    #[serde(rename = "completed")]
    Completed,
}

/// Interval type for the simulation.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum SimulationInterval {
    /// Hourly interval.
    #[serde(rename = "hourly")]
    Hourly,

    /// Daily interval.
    #[serde(rename = "daily")]
    Daily,

    /// Weekly interval.
    #[serde(rename = "weekly")]
    Weekly,

    /// Monthly interval.
    #[serde(rename = "monthly")]
    Monthly,
}

impl Simulation {
    /// Update the status of the simulation.   
    ///
    /// # Arguments
    ///
    /// * `status` - The new status of the simulation.
    pub fn update_status(&mut self, status: SimulationStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Run the simulation.
    ///
    /// # Arguments
    ///
    /// * `token` - The token used in the simulation.
    pub fn run(&mut self, token: &Token) {
        self.update_status(SimulationStatus::Running);

        let initial_supply = token.initial_supply();
        let mut users = User::generate(self.options.total_users, initial_supply);

        self.interval_reports = HashMap::default();

        let interval = match self.options.interval_type {
            SimulationInterval::Hourly => 1,
            SimulationInterval::Daily => 24,
            SimulationInterval::Weekly => 24 * 7,
            SimulationInterval::Monthly => 24 * 30,
        };

        for time in (0..self.options.duration * interval).step_by(interval as usize) {
            let report = self.simulate_interval(&mut users, interval);
            self.interval_reports.insert(time, report);
        }

        self.calculate_report(&users);

        self.update_status(SimulationStatus::Completed);
    }

    /// Simulate trades for a given interval.
    ///
    /// # Arguments
    ///
    /// * `users` - A list of users.
    /// * `interval` - Duration of the interval.
    ///
    /// # Returns
    ///
    /// A report of the simulation results for the interval.
    fn simulate_interval(&self, users: &mut [User], interval: u64) -> SimulationReport {
        let mut rng = rand::thread_rng();

        let mut trades = 0;
        let mut total_burned = 0.0;
        let mut total_new_tokens = 0.0;
        let mut report = SimulationReport::default();

        for _ in 0..interval {
            for user in users.iter_mut() {
                // Determine if the trade is successful
                if rng.gen_bool(0.5) {
                    // Simulate a successful trade
                    let trade_amount = rng.gen_range(0.0..user.balance);
                    user.balance -= trade_amount;
                    report.profit_loss += trade_amount;
                    report.successful_trades += 1;

                    // Apply burn rate
                    if let Some(burn_rate) = self.token.burn_rate {
                        let burned = trade_amount * burn_rate;
                        user.balance -= burned;
                        total_burned += burned;
                    }

                    // Apply inflation rate
                    if let Some(inflation_rate) = self.token.inflation_rate {
                        let new_tokens = trade_amount * inflation_rate;
                        user.balance += new_tokens;
                        total_new_tokens += new_tokens;
                    }
                } else {
                    // Simulate a failed trade
                    report.failed_trades += 1;
                }
                trades += 1;
            }
        }

        report.trades = trades;
        report.liquidity = report.calculate_liquidity(trades, interval);
        report.adoption_rate = report.calculate_adoption_rate(users);
        report.burn_rate = report.calculate_burn_rate(total_burned, users.len() as u64);
        report.user_retention = report.calculate_user_retention(users);
        report.market_volatility = self.options.market_volatility;
        report.network_activity = trades as f64 / interval as f64;
        report.token_distribution = users.iter().map(|u| u.balance).collect();
        report.inflation_rate =
            report.calculate_inflation_rate(total_new_tokens, users.len() as u64);

        report
    }

    /// Calculate the total report for the simulation.
    ///
    /// # Arguments
    ///
    /// * `users` - A list of users.
    fn calculate_report(&mut self, users: &[User]) {
        let mut report = SimulationReport {
            market_volatility: self.options.market_volatility,
            ..Default::default()
        };

        let mut total_burned = 0.0;
        let mut total_new_tokens = 0.0;

        for result in self.interval_reports.values() {
            report.profit_loss += result.profit_loss;
            report.trades += result.trades;
            report.successful_trades += result.successful_trades;
            report.failed_trades += result.failed_trades;

            total_burned += result.burn_rate * self.options.total_users as f64;
            total_new_tokens += result.inflation_rate * self.options.total_users as f64;
            report.liquidity += result.liquidity;
            report.adoption_rate += result.adoption_rate;
            report.user_retention += result.user_retention;
        }

        let total_intervals = self.interval_reports.len() as f64;

        report.liquidity /= total_intervals;
        report.adoption_rate /= total_intervals;
        report.user_retention /= total_intervals;
        report.token_distribution = users.iter().map(|u| u.balance).collect();
        report.burn_rate = total_burned / report.trades as f64;
        report.inflation_rate = total_new_tokens / report.trades as f64;
        report.network_activity = report.trades as f64 / self.options.duration as f64;

        self.report = report;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_status() {
        let mut simulation = Simulation {
            id: Uuid::new_v4(),
            name: "Test Simulation".to_string(),
            token: Token::default(),
            description: None,
            status: SimulationStatus::Running,
            options: SimulationOptions {
                duration: 30,
                total_users: 100,
                market_volatility: 0.5,
                interval_type: SimulationInterval::Daily,
            },
            interval_reports: HashMap::default(),
            report: SimulationReport::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        simulation.update_status(SimulationStatus::Completed);

        assert_eq!(simulation.status, SimulationStatus::Completed);
    }

    #[test]
    fn test_run() {
        let token = Token::default();
        let mut simulation = Simulation {
            id: Uuid::new_v4(),
            name: "Test Simulation".to_string(),
            token: token.to_owned(),
            description: None,
            status: SimulationStatus::Running,
            options: SimulationOptions {
                duration: 30,
                total_users: 100,
                market_volatility: 0.5,
                interval_type: SimulationInterval::Daily,
            },
            interval_reports: HashMap::default(),
            report: SimulationReport::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        simulation.run(&token);

        assert_eq!(simulation.status, SimulationStatus::Completed);
        assert_eq!(simulation.interval_reports.len(), 30);
    }
}

use std::{collections::HashMap, hash::BuildHasherDefault};

use chrono::{DateTime, Utc};
use rand::Rng;
use rust_decimal::{prelude::*, Decimal};
use serde::{Deserialize, Serialize};
use twox_hash::XxHash64;
use uuid::Uuid;

use crate::{SimulationReport, Token, User, DECIMAL_PRECISION};

/// Input parameters for a simulation.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimulationOptions {
    /// Duration of the simulation, depending on the interval type.
    pub duration: u64,

    /// Number of users in the simulation.
    pub total_users: u64,

    /// Volatility level. 0.0 is no volatility, 1.0 is maximum volatility.
    pub market_volatility: Decimal,

    /// Interval type for the simulation.
    pub interval_type: SimulationInterval,

    /// Transaction fee for each trade.
    pub transaction_fee: Option<Decimal>,
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
    pub fn run(&mut self, token: &mut Token) {
        self.update_status(SimulationStatus::Running);

        // Apply airdrop, if available
        let airdrop_amount = match token.airdrop_percentage {
            Some(percentage) => token.airdrop(percentage),
            None => Decimal::default(),
        };

        let mut users = User::generate(
            self.options.total_users,
            token.initial_supply(),
            token.initial_price,
        );

        // Distribute airdrop amount among users, if available
        if airdrop_amount.is_zero() {
            let airdrop_per_user = airdrop_amount / Decimal::new(users.len() as i64, 0);

            for user in &mut users {
                user.balance += airdrop_per_user.round_dp(DECIMAL_PRECISION);
            }
        }

        self.interval_reports = HashMap::default();

        let interval = self.get_interval();

        for time in (0..self.options.duration * interval).step_by(interval as usize) {
            // Process unlock events up to the current time
            let current_date = Utc::now() + chrono::Duration::hours(time as i64);
            token.process_unlocks(current_date);

            let report = self.process_interval(&mut users, interval);
            self.interval_reports.insert(time, report);
        }

        self.generate_report(&users);

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
    pub fn process_interval(&self, users: &mut [User], interval: u64) -> SimulationReport {
        let mut rng = rand::thread_rng();

        let mut trades = 0;
        let total_users = Decimal::new(users.len() as i64, 0);
        let mut total_burned = Decimal::default();
        let mut total_new_tokens = Decimal::default();
        let mut report = SimulationReport::default();

        for _ in 0..interval {
            for user in users.iter_mut() {
                // Skip users with zero balance
                if user.balance.is_zero() {
                    continue;
                }

                // Simulate a trade, 50% chance of success
                if rng.gen_bool(0.5) {
                    // Simulate a successful trade and randomise the fraction between 1% and 10% of the user's balance
                    let trade_fraction = rng.gen_range(0.01..0.1); //
                    let trade_amount = Decimal::from_f64(
                        rng.gen_range(0.0..(user.balance.to_f64().unwrap() * trade_fraction)),
                    )
                    .unwrap()
                    .round_dp(DECIMAL_PRECISION);

                    user.balance -= trade_amount;
                    report.profit_loss += trade_amount;
                    report.successful_trades += 1;

                    if let Some(burn_rate) = self.token.burn_rate {
                        let burned = trade_amount * burn_rate;
                        user.balance -= burned;
                        total_burned += burned;
                    }

                    if let Some(inflation_rate) = self.token.inflation_rate {
                        let new_tokens = trade_amount * inflation_rate;
                        user.balance += new_tokens;
                        total_new_tokens += new_tokens;
                    }

                    if let Some(fee) = self.options.transaction_fee {
                        user.balance -= trade_amount * fee.round_dp(DECIMAL_PRECISION);
                    }
                } else {
                    report.failed_trades += 1;
                }
                trades += 1;
            }
        }

        report.trades = trades;
        report.liquidity = report.calculate_liquidity(
            Decimal::new(trades as i64, 0),
            Decimal::new(interval as i64, 0),
        );
        report.adoption_rate = report.calculate_adoption_rate(users);
        report.burn_rate = report.calculate_burn_rate(total_burned, total_users);
        report.user_retention = report.calculate_user_retention(users);
        report.market_volatility = self.options.market_volatility;
        report.network_activity = trades / interval;
        report.token_distribution = users.iter().map(|u| u.balance).collect();
        report.inflation_rate = report.calculate_inflation_rate(total_new_tokens, total_users);

        report
    }

    /// Calculate the total report for the simulation.
    ///
    /// # Arguments
    ///
    /// * `users` - A list of users.
    pub fn generate_report(&mut self, users: &[User]) {
        let mut report = SimulationReport {
            market_volatility: self.options.market_volatility,
            ..Default::default()
        };

        let mut total_burned = Decimal::default();
        let mut total_new_tokens = Decimal::default();
        let total_users = Decimal::new(self.options.total_users as i64, 0);

        for result in self.interval_reports.values() {
            report.profit_loss += result.profit_loss;
            report.trades += result.trades;
            report.successful_trades += result.successful_trades;
            report.failed_trades += result.failed_trades;

            total_burned += result.burn_rate * total_users;
            total_new_tokens += result.inflation_rate * total_users;
            report.liquidity += result.liquidity;
            report.adoption_rate += result.adoption_rate;
            report.user_retention += result.user_retention;
        }

        let total_trades = Decimal::new(report.trades as i64, 0);
        let total_intervals = Decimal::new(self.interval_reports.len() as i64, 0);

        report.liquidity = (report.liquidity / total_intervals).round_dp(DECIMAL_PRECISION);
        report.adoption_rate = (report.adoption_rate / total_intervals).round_dp(DECIMAL_PRECISION);
        report.user_retention =
            (report.user_retention / total_intervals).round_dp(DECIMAL_PRECISION);
        report.token_distribution = users
            .iter()
            .map(|u| u.balance.round_dp(DECIMAL_PRECISION))
            .collect();
        report.burn_rate = report.calculate_burn_rate(total_burned, total_trades);
        report.inflation_rate = (total_new_tokens / total_trades).round_dp(DECIMAL_PRECISION);
        report.network_activity = report.trades / self.options.duration;

        self.report = report;
    }

    /// Get the interval for the simulation.
    ///
    /// # Returns
    ///
    /// The duration of the simulation interval.
    pub fn get_interval(&self) -> u64 {
        match self.options.interval_type {
            SimulationInterval::Hourly => 1,
            SimulationInterval::Daily => 24,
            SimulationInterval::Weekly => 24 * 7,
            SimulationInterval::Monthly => 24 * 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Simulation {
        Simulation {
            id: Uuid::new_v4(),
            name: "Test Simulation".to_string(),
            token: Token::default(),
            description: None,
            status: SimulationStatus::Running,
            options: SimulationOptions {
                duration: 30,
                total_users: 100,
                market_volatility: Decimal::new(5, 1),
                transaction_fee: None,
                interval_type: SimulationInterval::Daily,
            },
            interval_reports: HashMap::default(),
            report: SimulationReport::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_get_interval() {
        let daily_simulation = setup();
        assert_eq!(daily_simulation.get_interval(), 24);

        let mut hourly_simulation = setup();
        hourly_simulation.options.interval_type = SimulationInterval::Hourly;
        assert_eq!(hourly_simulation.get_interval(), 1);

        let mut weekly_simulation = setup();
        weekly_simulation.options.interval_type = SimulationInterval::Weekly;
        assert_eq!(weekly_simulation.get_interval(), 24 * 7);

        let mut monthly_simulation = setup();
        monthly_simulation.options.interval_type = SimulationInterval::Monthly;
        assert_eq!(monthly_simulation.get_interval(), 24 * 30);
    }

    #[test]
    fn test_update_status() {
        let mut simulation = setup();
        simulation.update_status(SimulationStatus::Completed);

        assert_eq!(simulation.status, SimulationStatus::Completed);
    }

    #[test]
    fn test_run() {
        let mut token = Token::default();
        let mut simulation = setup();

        simulation.run(&mut token);

        assert_eq!(simulation.status, SimulationStatus::Completed);
        assert_eq!(simulation.interval_reports.len(), 30);
    }
}

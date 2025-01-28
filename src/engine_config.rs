use rust_decimal::{prelude::*, Decimal};
use serde::{Deserialize, Serialize};

use crate::{SimulationError, SimulationInterval};

/// Input parameters for a simulation.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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

    /// Rate at which users adopt the token.
    pub adoption_rate: Option<Decimal>,

    /// Valuation model for the token.
    pub valuation_model: Option<ValuationModel>,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
/// Builder for creating a new simulation options.
pub struct SimulationOptionsBuilder {
    /// Duration of the simulation, depending on the interval type.
    duration: Option<u64>,

    /// Number of users in the simulation.
    total_users: Option<u64>,

    /// Volatility level. 0.0 is no volatility, 1.0 is maximum volatility.
    market_volatility: Option<f64>,

    /// Interval type for the simulation.
    interval_type: Option<SimulationInterval>,

    /// Transaction fee for each trade.
    transaction_fee: Option<f64>,

    /// Rate at which users adopt the token.
    adoption_rate: Option<f64>,

    /// Valuation model for the token.
    valuation_model: Option<ValuationModel>,
}

/// Valuation model for the token.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ValuationModel {
    /// Linear valuation model: valuation = users * initial_price.
    #[serde(rename = "linear")]
    Linear,

    /// Exponential valuation model: valuation = initial_price * e^(users / some_factor).
    #[serde(rename = "exponential")]
    Exponential,
}

impl SimulationOptionsBuilder {
    /// Create a new simulation options builder to configure the simulation.
    ///
    /// # Returns
    ///
    /// New simulation options builder.
    pub fn new() -> Self {
        SimulationOptionsBuilder::default()
    }

    /// Set the duration of the simulation.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the simulation.
    ///
    /// # Returns
    ///
    /// The simulation options builder.
    pub fn duration(mut self, duration: u64) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set the total number of users in the simulation.
    ///
    /// # Arguments
    ///
    /// * `total_users` - Total number of users in the simulation.
    ///
    /// # Returns
    ///
    /// The simulation options builder.
    pub fn total_users(mut self, total_users: u64) -> Self {
        self.total_users = Some(total_users);
        self
    }

    /// Set the market volatility level.
    ///
    /// # Arguments
    ///
    /// * `market_volatility` - Market volatility level.
    ///
    /// # Returns
    ///
    /// The simulation options builder.
    pub fn market_volatility(mut self, market_volatility: f64) -> Self {
        self.market_volatility = Some(market_volatility);
        self
    }

    /// Set the interval type for the simulation.
    ///
    /// # Arguments
    ///
    /// * `interval_type` - Interval type for the simulation.
    ///
    /// # Returns
    ///
    /// The simulation options builder.
    pub fn interval_type(mut self, interval_type: SimulationInterval) -> Self {
        self.interval_type = Some(interval_type);
        self
    }

    /// Set the transaction fee for each trade.
    ///
    /// # Arguments
    ///
    /// * `transaction_fee` - Transaction fee for each trade.
    ///
    /// # Returns
    ///
    /// The simulation options builder.
    pub fn transaction_fee(mut self, transaction_fee: f64) -> Self {
        self.transaction_fee = Some(transaction_fee);
        self
    }

    /// Set the rate at which users adopt the token.
    ///
    /// # Arguments
    ///
    /// * `adoption_rate` - Rate at which users adopt the token.
    ///
    /// # Returns
    ///
    /// The simulation options builder.
    pub fn adoption_rate(mut self, adoption_rate: f64) -> Self {
        self.adoption_rate = Some(adoption_rate);
        self
    }

    /// Set the valuation model for the token.
    ///
    /// # Arguments
    ///
    /// * `valuation_model` - Valuation model for the token.
    ///
    /// # Returns
    ///
    /// The simulation options builder.
    pub fn valuation_model(mut self, valuation_model: ValuationModel) -> Self {
        self.valuation_model = Some(valuation_model);
        self
    }

    /// Build the simulation options.
    ///
    /// # Returns
    ///
    /// Built simulation options or an error if required fields are missing.
    pub fn build(self) -> Result<SimulationOptions, SimulationError> {
        Ok(SimulationOptions {
            duration: self.duration.unwrap_or(7),
            total_users: self.total_users.ok_or(SimulationError::MissingTotalUsers)?,
            market_volatility: Decimal::from_f64(self.market_volatility.unwrap_or(0.5)).unwrap(),
            interval_type: self.interval_type.unwrap_or(SimulationInterval::Daily),
            transaction_fee: match self.transaction_fee {
                Some(fee) => Some(Decimal::from_f64(fee).ok_or(SimulationError::InvalidDecimal)?),
                None => None,
            },
            adoption_rate: match self.adoption_rate {
                Some(rate) => Some(Decimal::from_f64(rate).ok_or(SimulationError::InvalidDecimal)?),
                None => None,
            },
            valuation_model: self.valuation_model,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::SimulationInterval;
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_new_simulation_options_builder() {
        let builder = SimulationOptionsBuilder::new();

        assert_eq!(builder.duration, None);
        assert_eq!(builder.total_users, None);
        assert_eq!(builder.market_volatility, None);
        assert_eq!(builder.interval_type, None);
        assert_eq!(builder.transaction_fee, None);
        assert_eq!(builder.adoption_rate, None);
        assert_eq!(builder.valuation_model, None);
    }

    #[test]
    fn test_build_simulation_options_with_only_required() {
        let builder = SimulationOptionsBuilder::new();
        let options = builder
            .total_users(100)
            .market_volatility(0.5)
            .build()
            .unwrap();

        assert_eq!(options.duration, 7);
        assert_eq!(options.total_users, 100);
        assert_eq!(options.market_volatility, Decimal::new(5, 1));
        assert_eq!(options.interval_type, SimulationInterval::Daily);
        assert_eq!(options.transaction_fee, None);
        assert_eq!(options.adoption_rate, None);
        assert_eq!(options.valuation_model, None);
    }
    #[test]
    fn test_build_simulation_options() {
        let builder = SimulationOptionsBuilder::new();
        let options = builder
            .adoption_rate(1.0)
            .duration(10)
            .interval_type(SimulationInterval::Daily)
            .transaction_fee(0.01)
            .valuation_model(ValuationModel::Linear)
            .total_users(100)
            .market_volatility(0.5)
            .build()
            .unwrap();

        assert_eq!(options.duration, 10);
        assert_eq!(options.total_users, 100);
        assert_eq!(options.market_volatility, Decimal::new(5, 1));
        assert_eq!(options.interval_type, SimulationInterval::Daily);
        assert_eq!(options.transaction_fee, Some(Decimal::new(1, 2)));
        assert_eq!(options.adoption_rate, Some(Decimal::new(1, 0)));
        assert_eq!(options.valuation_model, Some(ValuationModel::Linear));
    }

    #[test]
    fn test_build_simulation_options_missing_total_users() {
        let builder = SimulationOptionsBuilder::new();
        let result = builder.build();

        assert_eq!(result, Err(SimulationError::MissingTotalUsers));
    }
}

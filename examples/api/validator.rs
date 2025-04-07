use rust_decimal::Decimal;
use tokenomics_simulator::{SimulationInterval, SimulationOptions, Token};

use crate::Exception;

/// Validator trait.
pub trait Validator {
    /// Validate the input data.
    ///
    /// # Returns
    ///
    /// Result of the validation.
    fn validate(&self) -> Result<(), Exception>;
}

impl Validator for Token {
    /// Validate the token input data.
    ///
    /// # Returns
    ///
    /// Result of the validation.
    fn validate(&self) -> Result<(), Exception> {
        if let Some(airdrop) = self.airdrop_percentage {
            if airdrop <= Decimal::default() || airdrop > Decimal::new(100, 0) {
                return Err(Exception::ValidationFailed(
                    "Airdrop percentage must be more 0 and less than or equal to 100.".to_string(),
                ));
            }
        }

        if self.initial_supply_percentage <= Decimal::default()
            || self.initial_supply_percentage > Decimal::new(100, 0)
        {
            return Err(Exception::ValidationFailed(
                "Initial supply percentage must be more than 0 and less than or equal to 100."
                    .to_string(),
            ));
        }

        Ok(())
    }
}

impl Validator for SimulationOptions {
    /// Validate the simulation options input data.
    ///
    /// # Returns
    ///
    /// Result of the validation.
    fn validate(&self) -> Result<(), Exception> {
        if self.total_users < 1 || self.total_users > 100000 {
            return Err(Exception::ValidationFailed(
                "Total users must be more than 0 and less than or equal to 100000.".to_string(),
            ));
        }

        if self.decimal_precision < 1 || self.decimal_precision > 18 {
            return Err(Exception::ValidationFailed(
                "Decimal precision must be more than or equal to 0 and less than or equal to 18."
                    .to_string(),
            ));
        }

        match self.interval_type {
            SimulationInterval::Daily => {
                if self.duration < 1 || self.duration > 365 {
                    return Err(Exception::ValidationFailed(
                        "Duration value must be more than 0 and less than or equal to 365."
                            .to_string(),
                    ));
                }
            }
            SimulationInterval::Hourly => {
                if self.duration < 1 || self.duration > 24 {
                    return Err(Exception::ValidationFailed(
                        "Duration value must be more than 0 and less than or equal to 24."
                            .to_string(),
                    ));
                }
            }
            SimulationInterval::Weekly => {
                if self.duration < 1 || self.duration > 52 {
                    return Err(Exception::ValidationFailed(
                        "Duration value must be more than 0 and less than or equal to 52."
                            .to_string(),
                    ));
                }
            }
            SimulationInterval::Monthly => {
                if self.duration < 1 || self.duration > 12 {
                    return Err(Exception::ValidationFailed(
                        "Duration value must be more than 0 and less than or equal to 12."
                            .to_string(),
                    ));
                }
            }
        };

        if self.market_volatility < Decimal::default()
            || self.market_volatility > Decimal::new(1, 0)
        {
            return Err(Exception::ValidationFailed(
                "Market volatility must be more than or equal to 0 and less than or equal to 1."
                    .to_string(),
            ));
        }

        if let Some(fee) = self.transaction_fee_percentage {
            if fee <= Decimal::default() || fee > Decimal::new(1, 0) {
                return Err(Exception::ValidationFailed(
                    "Transaction fee percentage must be more than 0 and less than or equal to 100."
                        .to_string(),
                ));
            }
        }

        Ok(())
    }
}

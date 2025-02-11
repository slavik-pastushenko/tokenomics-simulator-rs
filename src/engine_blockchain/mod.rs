//! # Engine blockchain module
//!
//! The engine blockchain module contains the different blockchain implementations which are used to simulate various blockchain data.
//! It is helpful when you do not know the exact transaction fee for a specific blockchain or you do not know which blockchain to use.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::SimulationError;

/// Ethereum blockchain module.
pub mod ethereum;

/// Solana blockchain module.
pub mod solana;

pub use ethereum::*;
pub use solana::*;

/// Transaction fee for each trade.
/// At the moment, the engine supports Ethereum and Solana blockchains.
/// If you want to simulate the transaction fee for a specific blockchain, you can use the predefined fees.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum SimulationTransactionFee {
    /// Custom transaction fee.
    /// It is useful when you want to simulate the transaction fee for a specific blockchain without using external API requests
    /// or when you want to use a custom transaction fee.
    Custom(f64),

    /// Transaction fee for Ethereum blockchain. The fee is in Gwei.
    /// The data is fetched from the external API.
    /// The required parameter is used to fetch the data from the external API.
    /// To create and use an API key, visit the [Etherscan API](https://docs.etherscan.io/getting-started/viewing-api-usage-statistics) documentation.
    Ethereum(String),

    /// Transaction fee for Solana blockchain. The fee is in lamports.
    /// The data is fetched from the external API.
    Solana,
}

/// Engine blockchain trait.
pub trait EngineBlockchain {
    /// Get the transaction fee.
    /// The transaction fee is used to calculate the total cost of the trade.
    ///
    /// # Arguments
    ///
    /// * `api_key` - API key for the external API.
    /// * `api_url` - API URL for the external API. If not provided, the default URL is used.
    ///
    /// # Returns
    ///
    /// `Decimal` - The transaction fee.
    fn get_fee_per_transaction(
        &self,
        api_key: Option<String>,
        api_url: Option<String>,
    ) -> Result<Decimal, SimulationError>;
}

use reqwest::{blocking::Client, StatusCode};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::{Deserialize, Serialize};

use super::EngineBlockchain;
use crate::SimulationError;

/// Solana blockchain implementation.
pub struct SolanaBlockChain;

/// Solana API response.
#[derive(Debug, Deserialize)]
struct SolanaResponse {
    /// The result of the API request.
    result: SolanaResult,
}

/// Solana response value.
#[derive(Debug, Deserialize)]
struct SolanaResult {
    /// The value of the API response.
    value: SolanaValue,
}

/// Solana value.
#[derive(Debug, Deserialize)]
struct SolanaValue {
    /// The fee calculator.
    fee_calculator: FeeCalculator,
}

/// Fee calculator.
#[derive(Debug, Deserialize)]
struct FeeCalculator {
    /// The lamports per signature.
    lamports_per_signature: u64,
}

/// Solana request body.
#[derive(Debug, Deserialize, Serialize)]
struct SolanaRequest {
    /// JSON RPC version.
    jsonrpc: String,

    /// ID of the request.
    id: u64,

    /// Method to get the recent blockhash.
    method: String,
}

impl EngineBlockchain for SolanaBlockChain {
    /// Get the transaction fee.
    /// The transaction fee is used to calculate the total cost of the trade.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Unused for Solana.
    /// * `api_url` - API URL for the external API.
    ///
    /// # Returns
    ///
    /// Transaction fee.
    fn get_fee_per_transaction(
        &self,
        _api_key: Option<String>,
        api_url: Option<String>,
    ) -> Result<Decimal, SimulationError> {
        let response: SolanaResponse = match Client::new()
            .post(api_url.unwrap_or("https://api.mainnet-beta.solana.com".to_string()))
            .json(&SolanaRequest {
                jsonrpc: "2.0".to_string(),
                id: 1,
                method: "getLatestBlockhash".to_string(),
            })
            .send()
        {
            Ok(response) => {
                if response.status() == StatusCode::UNAUTHORIZED {
                    return Err(SimulationError::InvalidApiKey);
                }

                match response.json::<SolanaResponse>() {
                    Ok(response) => response,
                    Err(_) => return Err(SimulationError::InvalidApiRequest),
                }
            }
            Err(_) => return Err(SimulationError::InvalidApiRequest),
        };

        let fee =
            response.result.value.fee_calculator.lamports_per_signature as f64 / 1_000_000_000.0;

        match Decimal::from_f64(fee) {
            Some(fee) => Ok(fee),
            None => Err(SimulationError::InvalidDecimal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn test_get_fee_per_transaction() {
        let mut mock_server = Server::new();
        let url = mock_server.url();
        let mock = mock_server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "result": {
                        "value": {
                            "fee_calculator": {
                                "lamports_per_signature": 5000
                            }
                        }
                    }
                }"#,
            )
            .create();

        let fee = SolanaBlockChain.get_fee_per_transaction(None, Some(url));

        mock.assert();
        assert_eq!(fee.unwrap(), Decimal::from_f64(0.000005).unwrap());
    }
}

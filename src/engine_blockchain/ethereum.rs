use reqwest::StatusCode;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::Deserialize;

use crate::SimulationError;

use super::EngineBlockchain;

/// Ethereum blockchain implementation.
pub struct EthereumBlockChain;

/// Etherscan response.
/// The response from the Etherscan API. The response contains the gas price in wei.
#[derive(Debug, Deserialize)]
struct EtherscanResponse {
    /// The result of the API request.
    result: String,
}

impl EngineBlockchain for EthereumBlockChain {
    /// Get the transaction fee.
    /// The transaction fee is used to calculate the total cost of the trade.
    /// To create and use an API key, visit the [Etherscan API](https://docs.etherscan.io/getting-started/viewing-api-usage-statistics) documentation.
    ///
    /// # Arguments
    ///
    /// * `api_key` - API key for the external API.
    /// * `api_url` - API URL for the external API.
    ///
    /// # Returns
    ///
    /// Transaction fee.
    fn get_fee_per_transaction(
        &self,
        api_key: Option<String>,
        api_url: Option<String>,
    ) -> Result<Decimal, SimulationError> {
        let url = format!(
            "{}/api?module=proxy&action=eth_gasPrice&apikey={}",
            api_url.unwrap_or("https://api.etherscan.io".to_string()),
            api_key.unwrap_or_default()
        );
        let response = match reqwest::blocking::get(&url) {
            Ok(response) => {
                if response.status() == StatusCode::UNAUTHORIZED {
                    return Err(SimulationError::InvalidApiKey);
                }

                match response.json::<EtherscanResponse>() {
                    Ok(response) => response,
                    Err(_) => return Err(SimulationError::InvalidApiRequest),
                }
            }
            Err(_) => return Err(SimulationError::InvalidApiRequest),
        };

        let gas_price_wei = if response.result.starts_with("0x") {
            match u64::from_str_radix(&response.result[2..], 16) {
                Ok(gas_price_wei) => gas_price_wei,
                Err(_) => return Err(SimulationError::InvalidApiConversion),
            }
        } else {
            match response.result.parse::<u64>() {
                Ok(gas_price_wei) => gas_price_wei,
                Err(_) => return Err(SimulationError::InvalidApiConversion),
            }
        };

        let gas_price_eth = gas_price_wei as f64 / 1_000_000_000.0;

        match Decimal::from_f64(gas_price_eth) {
            Some(gas_price_eth) => Ok(gas_price_eth),
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
        let api_key = "api-key".to_string();
        let mut mock_server = Server::new();
        let mock = mock_server
            .mock(
                "GET",
                "/api?module=proxy&action=eth_gasPrice&apikey=api-key",
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result":"0x430e23400"}"#)
            .create();

        let result =
            EthereumBlockChain.get_fee_per_transaction(Some(api_key), Some(mock_server.url()));

        mock.assert();
        assert_eq!(result.unwrap(), Decimal::from_f64(18.0).unwrap());
    }

    #[test]
    fn test_get_fee_per_transaction_invalid_request() {
        let api_key = "api-key".to_string();
        let mut mock_server = Server::new();
        let mock = mock_server
            .mock(
                "GET",
                "/api?module=proxy&action=eth_gasPrice&apikey=api-key",
            )
            .with_status(400)
            .with_header("content-type", "application/json")
            .create();

        let result =
            EthereumBlockChain.get_fee_per_transaction(Some(api_key), Some(mock_server.url()));

        mock.assert();
        assert_eq!(result, Err(SimulationError::InvalidApiRequest));
    }

    #[test]
    fn test_get_fee_per_transaction_unauthorized() {
        let api_key = "api-key".to_string();
        let mut mock_server = Server::new();
        let mock = mock_server
            .mock(
                "GET",
                "/api?module=proxy&action=eth_gasPrice&apikey=api-key",
            )
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result":"0x430e23400"}"#)
            .create();

        let result =
            EthereumBlockChain.get_fee_per_transaction(Some(api_key), Some(mock_server.url()));

        mock.assert();
        assert_eq!(result, Err(SimulationError::InvalidApiKey));
    }

    #[test]
    fn test_get_fee_per_transaction_invalid_value() {
        let api_key = "api-key".to_string();
        let mut mock_server = Server::new();
        let mock = mock_server
            .mock(
                "GET",
                "/api?module=proxy&action=eth_gasPrice&apikey=api-key",
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"result":"invalid"}"#)
            .create();

        let result =
            EthereumBlockChain.get_fee_per_transaction(Some(api_key), Some(mock_server.url()));

        mock.assert();
        assert_eq!(result, Err(SimulationError::InvalidApiConversion));
    }

    #[test]
    fn test_get_fee_per_transaction_invalid_response() {
        let api_key = "api-key".to_string();
        let mut mock_server = Server::new();
        let mock = mock_server
            .mock(
                "GET",
                "/api?module=proxy&action=eth_gasPrice&apikey=api-key",
            )
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"random":"0"}"#)
            .create();

        let result =
            EthereumBlockChain.get_fee_per_transaction(Some(api_key), Some(mock_server.url()));

        mock.assert();
        assert_eq!(result, Err(SimulationError::InvalidApiRequest));
    }
}

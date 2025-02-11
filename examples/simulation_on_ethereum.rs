use tokenomics_simulator::{
    Simulation, SimulationError, SimulationOptionsBuilder, SimulationTransactionFee,
};

fn main() -> Result<(), SimulationError> {
    // Build a new token
    let token = Simulation::token_builder()
        .name("EthereumToken".to_string())
        .symbol("ETHK".to_string())
        .total_supply(1_000_000)
        .airdrop_percentage(5.0)
        .burn_rate(1.0)
        .build()?;

    // Build simulation options with Ethereum transaction fee
    let options = SimulationOptionsBuilder::new()
        .total_users(100)
        .market_volatility(0.7)
        // To use the Ethereum transaction fee, you need to provide your Etherscan API key.
        // To create and use an API key, visit the Etherscan API documentation (https://docs.etherscan.io/getting-started/viewing-api-usage-statistics).
        .transaction_fee(SimulationTransactionFee::Ethereum(
            "your-etherscan-api-key".to_string(),
        ))
        .build()?;

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Ethereum Transaction Fee Simulation".to_string())
        .description("A simulation using Ethereum transaction fees".to_string())
        .token(token)
        .options(options)
        .build()?;

    // Run the simulation
    simulation.run()?;

    // Print the final simulation report
    println!("Final report: {:#?}", simulation.report);

    Ok(())
}

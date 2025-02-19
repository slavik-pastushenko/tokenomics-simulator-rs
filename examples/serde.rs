use tokenomics_simulator::{Simulation, SimulationError, Token};

fn main() -> Result<(), SimulationError> {
    // Build a custom token
    let token = serde_json::from_str::<Token>(
        r#"
        {
            "id": "a843d393-8e25-4e2f-8684-d203abd99ac6",
            "name": "Custom Token",
            "symbol": "CT",
            "decimals": 18,
            "total_supply": 1000000,
            "current_supply": 100,
            "initial_supply_percentage": 0.1,
            "inflation_rate": 0.0,
            "burn_rate": 0.2,
            "initial_price": 1.0,
            "airdrop_percentage": 0.0
        }
        "#,
    )
    .unwrap();

    // Build the simulation options
    let options = Simulation::options_builder()
        .total_users(200)
        .market_volatility(0.3)
        .build()?;

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Custom Token Simulation".to_string())
        .description("A simulation with a custom token".to_string())
        .token(token)
        .options(options)
        .build()?;

    // Run the simulation
    simulation.run()?;

    // Serialize the simulation to a JSON string
    let json = serde_json::to_string(&simulation).unwrap();

    // Print the JSON string
    println!("{}", json);

    Ok(())
}

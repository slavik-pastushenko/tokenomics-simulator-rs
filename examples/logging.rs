use tokenomics_simulator::{Simulation, SimulationError};

fn main() -> Result<(), SimulationError> {
    // Initialize the logger
    env_logger::init();

    // Build a custom token
    let token = Simulation::token_builder()
        .name("CustomToken".to_string())
        .symbol("CTK".to_string())
        .total_supply(500_000)
        .airdrop_percentage(10.0)
        .burn_rate(2.0)
        .build()?;

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

    Ok(())
}

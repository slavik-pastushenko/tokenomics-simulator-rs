use tokenomics_simulator::{Simulation, SimulationError};

fn main() -> Result<(), SimulationError> {
    // Build a new token
    let token = Simulation::token_builder()
        .name("AdvancedToken".to_string())
        .symbol("ATK".to_string())
        .total_supply(1_000_000)
        .airdrop_percentage(5.0)
        .burn_rate(1.0)
        .build()?;

    // Build advanced simulation options
    let options = Simulation::options_builder()
        .total_users(100)
        .market_volatility(0.7)
        .build()?;

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Advanced Options Simulation".to_string())
        .description("A simulation with advanced options".to_string())
        .token(token)
        .options(options)
        .build()?;

    // Run the simulation
    simulation.run()?;

    // Print the final simulation report
    println!("Final report: {:#?}", simulation.report);

    Ok(())
}

use tokenomics_simulator::{Simulation, SimulationError};

fn main() -> Result<(), SimulationError> {
    // Build a new token
    let token = Simulation::token_builder()
        .name("BasicToken".to_string())
        .symbol("BTK".to_string())
        .total_supply(1_000_000)
        .airdrop_percentage(5.0)
        .burn_rate(1.0)
        .build()?;

    // Build the simulation options
    let options = Simulation::options_builder()
        .total_users(100)
        .market_volatility(0.5)
        .build()?;

    // Build a new simulation with the token and options
    let mut simulation = Simulation::builder()
        .name("Basic Simulation".to_string())
        .description("A basic simulation example".to_string())
        .token(token)
        .options(options)
        .build()?;

    // Run the simulation
    simulation.run()?;

    // Print the final simulation report
    println!("Final report: {:#?}", simulation.report);

    Ok(())
}

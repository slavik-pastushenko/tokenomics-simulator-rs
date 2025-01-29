use tokenomics_simulator::{Simulation, SimulationError};

fn main() -> Result<(), SimulationError> {
    // Build a new token
    let token = Simulation::token_builder()
        .name("MultiSimToken".to_string())
        .symbol("MST".to_string())
        .total_supply(1_000_000)
        .airdrop_percentage(5.0)
        .burn_rate(1.0)
        .build()?;

    // Build the first set of simulation options
    let options1 = Simulation::options_builder()
        .total_users(100)
        .market_volatility(0.5)
        .build()?;

    // Build the second set of simulation options
    let options2 = Simulation::options_builder()
        .total_users(200)
        .market_volatility(0.3)
        .build()?;

    // Build and run the first simulation
    let mut simulation1 = Simulation::builder()
        .name("First Simulation".to_string())
        .description("The first simulation".to_string())
        .token(token.clone())
        .options(options1)
        .build()?;

    simulation1.run()?;
    println!("First simulation final report: {:#?}", simulation1.report);

    // Build and run the second simulation
    let mut simulation2 = Simulation::builder()
        .name("Second Simulation".to_string())
        .description("The second simulation".to_string())
        .token(token)
        .options(options2)
        .build()?;

    simulation2.run()?;
    println!("Second simulation final report: {:#?}", simulation2.report);

    Ok(())
}

use clap::{Arg, Command};
use tokenomics_simulator::{Simulation, SimulationError};

fn main() -> Result<(), SimulationError> {
    let matches = Command::new("Tokenomics Simulator CLI")
        .version("1.0")
        .author("simetrics <simetricsio@gmail.com>")
        .about("Run tokenomics simulation")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Set the name of the token")
                .required(true),
        )
        .arg(
            Arg::new("symbol")
                .short('s')
                .long("symbol")
                .value_name("SYMBOL")
                .help("Set the symbol of the token")
                .required(true),
        )
        .arg(
            Arg::new("total_supply")
                .short('t')
                .long("total_supply")
                .value_name("TOTAL_SUPPLY")
                .help("Set the total supply of the token")
                .required(true),
        )
        .arg(
            Arg::new("airdrop_percentage")
                .short('a')
                .long("airdrop_percentage")
                .value_name("AIRDROP_PERCENTAGE")
                .help("Set the airdrop percentage of the token")
                .required(true),
        )
        .arg(
            Arg::new("burn_rate")
                .short('b')
                .long("burn_rate")
                .value_name("BURN_RATE")
                .help("Set the burn rate of the token")
                .required(true),
        )
        .arg(
            Arg::new("total_users")
                .short('u')
                .long("total_users")
                .value_name("TOTAL_USERS")
                .help("Set the total number of users for the simulation")
                .required(true),
        )
        .arg(
            Arg::new("market_volatility")
                .short('v')
                .long("market_volatility")
                .value_name("MARKET_VOLATILITY")
                .help("Set the market volatility for the simulation")
                .required(true),
        )
        .get_matches();

    let token = Simulation::token_builder()
        .name(matches.get_one::<String>("name").unwrap().to_string())
        .symbol(matches.get_one::<String>("symbol").unwrap().to_string())
        .total_supply(
            matches
                .get_one::<String>("total_supply")
                .unwrap()
                .parse()
                .unwrap(),
        )
        .airdrop_percentage(
            matches
                .get_one::<String>("airdrop_percentage")
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        )
        .burn_rate(
            matches
                .get_one::<String>("burn_rate")
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        )
        .build()?;

    let options = Simulation::options_builder()
        .total_users(
            matches
                .get_one::<String>("total_users")
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        )
        .market_volatility(
            matches
                .get_one::<String>("market_volatility")
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        )
        .build()?;

    let mut simulation = Simulation::builder()
        .name("Simulation".to_string())
        .description("Initial simulation".to_string())
        .token(token)
        .options(options)
        .build()?;

    simulation.run()?;

    println!("Final report: {:#?}", simulation.report);

    Ok(())
}

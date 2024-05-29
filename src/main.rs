use clap::{Arg, Command};
use reqwest::Error;
use serde::Deserialize;
use tokio;

#[derive(Deserialize)]
struct Rates {
    rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = Command::new("Currency Exchange CLI")
        .version("1.0") 
        .about("Fetches exchange rates for different currencies")
        .arg(Arg::new("base")
            .short('b')
            .long("base")
            .num_args(1)
            .required(true)
            .help("The base currency"))
        .arg(Arg::new("target")
            .short('t')
            .long("target")
            .num_args(1)
            .required(true)
            .help("The target currency"))
        .get_matches();

    let base_currency = matches.get_one::<String>("base").expect("Base currency is required");
    let target_currency = matches.get_one::<String>("target").expect("Target currency is required");

    let api_url = format!("https://api.exchangerate-api.com/v4/latest/{}", base_currency);
    let response = reqwest::get(&api_url).await?;
    let rates: Rates = response.json().await?;

    if let Some(rate) = rates.rates.get(target_currency) {
        println!("1 {} = {} {}", base_currency, rate, target_currency);
    } else {
        println!("Exchange rate not found for {} to {}", base_currency, target_currency);
    }

    Ok(())
}

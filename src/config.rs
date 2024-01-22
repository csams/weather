use std::env;
use std::error::Error;

use clap::Parser;

use crate::error;

#[derive(Parser)]
#[command(name = "CLI Weather")]
#[command(version = "1.0")]
#[command(about = "Forecasts on the CLI", long_about = None)]
pub struct Options {
    #[arg(long)]
    pub alerts: bool,
    #[arg(long)]
    pub hourly: bool,
    #[arg(long)]
    pub verbose: bool,
    pub address: Option<String>,
}

pub struct Config {
    pub alerts: bool,
    pub hourly: bool,
    pub verbose: bool,
    pub address: String,
}

impl Config {
    pub fn build(o: Options) -> Result<Config, Box<dyn Error>> {
        if let Some(address) = o.address.or(env::var("WX_DEFAULT_ADDRESS").ok()) {
            Ok(Config {
                alerts: o.alerts,
                hourly: o.hourly,
                verbose: o.verbose,
                address,
            })
        } else {
            error::boxed_err("Pass an address or set WX_DEFAULT_ADDRESS")
        }
    }
}

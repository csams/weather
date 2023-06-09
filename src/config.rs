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
    pub hourly: bool,
    pub address: Option<String>,
}

pub struct Config {
    pub hourly: bool,
    pub address: String,
}

impl Config {
    pub fn build(o: Options) -> Result<Config, Box<dyn Error>> {
        if let Some(address) = o.address.or(env::var("WX_DEFAULT_ADDRESS").ok()) {
            Ok(Config {
                hourly: o.hourly,
                address,
            })
        } else {
            error::boxed_err("Pass an address or set WX_DEFAULT_ADDRESS")
        }
    }
}

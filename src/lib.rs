//! weather is a library for fetching multi-day forecast information from api.weather.gov and
//! rendering it.

mod client;
pub mod config;
pub mod error;
pub mod forecast;
pub mod hour;
pub mod lookup;
pub mod style;
pub mod week;

use client::Request::*;
use std::error::Error;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let forecast_info = lookup::find(cfg.address.as_str())?;

    let url = if cfg.hourly {
        forecast_info.endpoints.hourly_url
    } else {
        forecast_info.endpoints.weekly_url
    };

    let doc: forecast::Doc = client::fetch(URL(url.as_str()))?;

    let render = if cfg.hourly {
        hour::render
    } else {
        week::render
    };

    println!("{}", render(&doc, style::elegant()));
    println!("{}", forecast_info.address);
    Ok(())
}

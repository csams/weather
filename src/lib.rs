//! weather is a library for loading multi-day forecast information from api.weather.gov and
//! rendering it.

use std::error::Error;
use client::Request::*;

pub mod client;
pub mod config;
pub mod error;
pub mod forecast;
pub mod hour;
pub mod lookup;
pub mod style;
pub mod week;

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

    let style = style::elegant();
    println!("{}", render(&doc, style));
    println!("{}", forecast_info.address);
    Ok(())
}

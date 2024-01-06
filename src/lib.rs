//! weather is a library for fetching multi-day forecast information from `api.weather.gov` and
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
use retry::delay::{jitter, Exponential};
use std::error::Error;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let res = retry::retry(Exponential::from_millis(1000).map(jitter).take(3), || {
        lookup::find(cfg.address.as_str()).and_then(|forecast_info|{
            let url = if cfg.hourly {
		forecast_info.endpoints.hourly_url
            } else {
		forecast_info.endpoints.weekly_url
            };

	    client::fetch(URL(url.as_str())).and_then(|doc|{
		let render = if cfg.hourly {
		    hour::render
		} else {
		    week::render
		};

		println!("{}", render(&doc, style::elegant()));
		println!("Forecast for: {}", forecast_info.address);
		Ok(())
	    })
	})
    });
    match res {
	Ok(r) => Ok(r),
	Err(e) => Err(e.error),
    }
}

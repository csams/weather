//! weather is a library for fetching multi-day forecast information from `api.weather.gov` and
//! rendering it.

pub mod alerts;
mod client;
pub mod config;
pub mod error;
pub mod forecast;
pub mod hourly;
pub mod location;
pub mod style;
pub mod weekly;

use retry::delay::{jitter, Exponential};
use std::error::Error;

pub fn run(cfg: config::Config) -> Result<(), Box<dyn Error>> {
    let res = retry::retry(Exponential::from_millis(1000).map(jitter).take(3), || {
        location::lookup(cfg.address.as_str()).and_then(|loc| {
            if cfg.alerts {
                alerts::lookup(&loc).and_then(|doc| {
                    println!("{}", alerts::render(&doc, cfg.verbose));
                    println!("Alerts for: {}", loc.address);
                    Ok(())
                })
            } else {
                forecast::lookup(&loc, cfg.hourly).and_then(|doc| {
                    let render = if cfg.hourly {
                        hourly::render
                    } else {
                        weekly::render
                    };

                    println!("{}", render(&doc, style::elegant()));
                    println!("Forecast for: {}", loc.address);
                    Ok(())
                })
            }
        })
    });

    // retry wraps errors, so we have to rebox them.
    match res {
        Ok(r) => Ok(r),
        Err(e) => Err(e.error),
    }
}

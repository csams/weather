//! weather is a library for loading multi-day forecast information from api.weather.gov and
//! rendering it.
//!

mod client;
pub mod hour;
pub mod model;
pub mod style;
pub mod week;

pub use client::Client;

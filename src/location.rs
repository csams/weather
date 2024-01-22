use std::error::Error;

use serde::Deserialize;

use crate::client;
use crate::client::Request::*;
use crate::error;

/// Looks up latitude and longitude of the resolved street address for the given query.
pub fn lookup(query: &str) -> Result<Location, Box<dyn Error>> {
    // benchmark = 4 is the identifier for the "current" dataset.
    let doc: CoordDoc = client::fetch(Query(
        "https://geocoding.geo.census.gov/geocoder/locations/onelineaddress",
        &vec![("address", query), ("benchmark", "4"), ("format", "json")],
    ))?;

    if doc.result.address_matches.len() == 0 {
        error::boxed_err("No matching addresses")
    } else {
        // just take the first one
        Ok(doc.result.address_matches[0].to_owned())
    }
}

// Ancillary types for parsing the json

#[derive(Debug, Deserialize)]
struct CoordDoc {
    pub result: Results,
}

#[derive(Debug, Deserialize)]
struct Results {
    #[serde(rename = "addressMatches")]
    pub address_matches: Vec<Location>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    #[serde(rename = "matchedAddress")]
    pub address: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}

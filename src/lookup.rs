use std::error::Error;

use serde::Deserialize;

use crate::client;
use crate::client::Request::*;
use crate::error;

/// Contains the resolved address and the urls to use for getting weekly and hourly forecast data.
pub struct ForecastInfo {
    pub address: String,
    pub endpoints: Endpoints,
}

#[derive(Debug, Deserialize)]
pub struct Endpoints {
    #[serde(rename = "forecastHourly")]
    pub hourly_url: String,

    #[serde(rename = "forecast")]
    pub weekly_url: String,
}

/// find the forecast information for the given street address query.
pub fn find(query: &str) -> Result<ForecastInfo, Box<dyn Error>> {
    let location = resolve_location(query)?;
    lookup_forecast_info(location)
}

/// Looks up latitude and longitude of the resolved street address for the given query.
fn resolve_location(query: &str) -> Result<Location, Box<dyn Error>> {
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

/// Looks up the forecast endpoints to use for the given resolved location.
fn lookup_forecast_info(location: Location) -> Result<ForecastInfo, Box<dyn Error>> {
    let url = format!(
        "https://api.weather.gov/points/{},{}",
        location.coordinates.y, location.coordinates.x
    );
    let doc: PointsDoc = client::fetch(URL(url.as_str()))?;
    Ok(ForecastInfo {
        address: location.address,
        endpoints: doc.properties,
    })
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
struct Location {
    #[serde(rename = "matchedAddress")]
    pub address: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, Deserialize, Clone)]
struct Coordinates {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
struct PointsDoc {
    pub properties: Endpoints,
}

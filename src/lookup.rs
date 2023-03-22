use serde::Deserialize;
use std::error::Error;

use crate::client;
use crate::client::Request::*;
use crate::error;

/// find the forecast information for the given street address. It contains the resolved address
/// and the urls to use for getting weekly and hourly forecast data.
pub fn find(query: &str) -> Result<ForecastInfo, Box<dyn Error>> {
    let address = resolve_address(query)?;
    lookup_forecast_info(address)
}

/// Looks up latitude and longitude of the resolved street address for the given query.
pub fn resolve_address(query: &str) -> Result<Location, Box<dyn Error>> {
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

/// Looks up the forecast endpoints to use for the given resolved address.
pub fn lookup_forecast_info(location: Location) -> Result<ForecastInfo, Box<dyn Error>> {
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

#[derive(Debug, Deserialize)]
pub struct CoordDoc {
    pub result: Results,
}

#[derive(Debug, Deserialize)]
pub struct Results {
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

#[derive(Debug, Deserialize)]
pub struct PointsDoc {
    pub properties: Endpoints,
}

#[derive(Debug, Deserialize)]
pub struct Endpoints {
    #[serde(rename = "forecastHourly")]
    pub hourly_url: String,

    #[serde(rename = "forecast")]
    pub weekly_url: String,
}

pub struct ForecastInfo {
    pub address: String,
    pub endpoints: Endpoints,
}

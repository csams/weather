use serde::Deserialize;
use std::error::Error;

use crate::client;
use crate::client::Request::*;
use crate::location::Location;

/// lookup finds the alerts for a point
pub fn lookup(loc: &Location) -> Result<Doc, Box<dyn Error>> {
    client::fetch(Query(
        "https://api.weather.gov/alerts/active",
        &vec![
            (
                "point",
                format!("{},{}", loc.coordinates.y, loc.coordinates.x).as_str(),
            ),
            ("status", "actual"),
            ("message_type", "alert,update"),
            ("urgency", "Immediate,Expected,Future"),
            ("severity", "Extreme,Severe,Moderate"),
            ("certainty", "Observed,Likely,Possible"),
            ("limit", "25"),
        ],
    ))
}

/// render creates a string from the alerts that's ready to print to the terminal
pub fn render(doc: &Doc, verbose: bool) -> String {
    let mut rows = vec![];

    doc.features.iter().for_each(|feature| {
        let props = &feature.properties;
        rows.push(format!("--------------------"));
        rows.push(format!("{}", props.headline));
        rows.push(format!("{} | {} | {}", props.severity, props.certainty, props.urgency));
        rows.push(String::from(""));
        rows.push(format!("{}", props.description));
        rows.push(format!("{}", props.instruction));
        rows.push(String::from(""));
        if verbose {
            let mut areas = props.area_desc.as_str().split("; ").collect::<Vec<_>>();
            areas.sort();
            let area_desc = areas.join("\n");
            rows.push(format!("{}", area_desc));
            rows.push(String::from(""));
        }
    });
    rows.join("\n")
}

/// Doc is the top level type returned from api.weather.gov's forecast endpoint. It contains a lot
/// of data, but we only care about the properties object of individual features.
#[derive(Debug, Deserialize)]
pub struct Doc {
    pub features: Vec<Feature>,
}

/// Feature contains blah
#[derive(Debug, Deserialize)]
pub struct Feature {
    pub properties: Properties,
}

/// Properties contains all of the information we care about
#[derive(Debug, Deserialize)]
pub struct Properties {
    pub effective: String,
    pub onset: String,
    pub expires: String,
    pub ends: String,
    pub status: String,

    #[serde(rename = "messageType")]
    pub message_type: String,
    pub category: String,
    pub severity: String,
    pub certainty: String,
    pub urgency: String,
    pub event: String,
    pub headline: String,

    #[serde(rename = "areaDesc")]
    pub area_desc: String,

    #[serde(rename = "senderName")]
    pub sender_name: String,
    pub description: String,
    pub instruction: String,
}

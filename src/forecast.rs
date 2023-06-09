use serde::Deserialize;

/// Doc is the top level type returned from api.weather.gov's forecast endpoint. It contains a lot
/// of data, but we only care about the properties object.
#[derive(Debug, Deserialize)]
pub struct Doc {
    pub properties: Properties,
}

/// Properties contains lots of info, but we only care about forecast periods.
#[derive(Debug, Deserialize)]
pub struct Properties {
    pub periods: Vec<Period>,
}

/// Period contains forecast information for one 12 hour period.
#[derive(Debug, Deserialize)]
pub struct Period {
    pub name: String,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "endTime")]
    pub end_time: String,

    #[serde(rename = "isDaytime")]
    pub is_daytime: bool,

    pub temperature: i32,

    #[serde(rename = "probabilityOfPrecipitation")]
    pub probability_of_precipitation: TypedField,

    #[serde(rename = "windSpeed")]
    pub wind_speed: String,

    #[serde(rename = "windDirection")]
    pub wind_direction: String,

    #[serde(rename = "shortForecast")]
    pub short_forecast: String,

    #[serde(rename = "detailedForecast")]
    pub detailed_forecast: String,
}

/// Some fields are objects that contain a value and a unit. We use the default units and
/// only care about the value.
#[derive(Debug, Deserialize)]
pub struct TypedField {
    pub value: Option<i32>,
}

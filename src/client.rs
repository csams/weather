use serde::Deserialize;
use std::error::Error;

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::blocking::Client::new(),
        }
    }

    /// load fetches data from api.weather.gov and parses it into some `Deserializable` object.
    pub fn load<T>(&self, url: &str) -> Result<T, Box<dyn Error>>
    where
        for<'a> T: Deserialize<'a>,
    {
        let doc: T = self
            .client
            .get(url)
            .header("User-Agent", "weather client")
            .send()?
            .json()?;
        Ok(doc)
    }
}

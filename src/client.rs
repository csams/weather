use std::error::Error;

use serde::de::DeserializeOwned;

type QueryParams<'a> = Vec<(&'a str, &'a str)>;

/// A fetch request is either a basic URL or a URL with some query parameters.
pub enum Request<'a> {
    URL(&'a str),
    Query(&'a str, &'a QueryParams<'a>),
}

/// Get a json document from a URL and parse it into an object.
// pub fn fetch<CoordDoc>(req: Request) -> Result<T, Box<dyn Error>>
pub fn fetch<T: DeserializeOwned>(req: Request) -> Result<T, Box<dyn Error>>
{
    let c = reqwest::blocking::Client::new();
    let obj: T = match req {
        Request::URL(url) => c.get(url),
        Request::Query(url, query) => c.get(url).query(query),
    }
    .header("User-Agent", "weather client")
    .send()?
    .json()?;

    Ok(obj)
}

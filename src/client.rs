use std::error::Error;

use serde::Deserialize;

type QueryParams<'a> = Vec<(&'a str, &'a str)>;

/// A fetch request is either a basic URL or a URL with some query parameters.
pub enum Request<'a> {
    URL(&'a str),
    Query(&'a str, &'a QueryParams<'a>),
}

/// Get a json document from a URL and parse it into an object.
pub fn fetch<T>(req: Request) -> Result<T, Box<dyn Error>>
where
    for<'a> T: Deserialize<'a>,
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

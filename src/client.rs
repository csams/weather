use serde::Deserialize;
use std::error::Error;

pub enum Request<'a> {
    URL(&'a str),
    Query(&'a str, &'a Vec<(&'a str, &'a str)>),
}

pub fn fetch<T>(req: Request) -> Result<T, Box<dyn Error>>
where
    for<'a> T: Deserialize<'a>,
{
    let c = reqwest::blocking::Client::new();
    let req = match req {
        Request::URL(url) => c.get(url),
        Request::Query(url, query) => c.get(url).query(query),
    };

    let doc: T = req.header("User-Agent", "weather client").send()?.json()?;
    Ok(doc)
}

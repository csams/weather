use std::{error::Error, fmt};

#[derive(Debug)]
pub struct WxError {
    pub msg: String,
}

pub fn new(m: &str) -> WxError {
    WxError { msg: m.to_owned() }
}

pub fn boxed_err<T>(m: &str) -> Result<T, Box<dyn Error>> {
    Err(Box::new(new(m)))
}

impl Error for WxError {}

impl fmt::Display for WxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = &self.msg;
        write!(f, "{m}")
    }
}

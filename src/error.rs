use std::{error::Error, fmt};

/// convert a str reference to a boxed custom error type to make generic error handling simplier.
pub fn boxed_err<T>(m: &str) -> Result<T, Box<dyn Error>> {
    Err(Box::new(WxError { msg: m.to_owned() }))
}

#[derive(Debug)]
struct WxError {
    pub msg: String,
}

impl Error for WxError {}

impl fmt::Display for WxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = &self.msg;
        write!(f, "{m}")
    }
}

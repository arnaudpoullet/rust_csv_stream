use std::error::Error;

use std::string::FromUtf8Error;
use csv::{IntoInnerError, Writer};

#[derive(Debug)]
pub struct ResponseError(pub String);

impl Error for ResponseError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

impl From<String> for ResponseError {
    fn from(err: String) -> Self{
        ResponseError(format!("{:?}", err))
    }
}

impl From<csv::Error> for ResponseError {
    fn from(err: csv::Error) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

impl From<std::io::Error> for ResponseError {
    fn from(err: std::io::Error) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

impl From<hyper::Error> for ResponseError {
    fn from(err: hyper::Error) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

impl From<FromUtf8Error> for ResponseError {
    fn from(err: FromUtf8Error) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

impl From<IntoInnerError<Writer<Vec<u8>>>> for ResponseError {
    fn from(err: IntoInnerError<Writer<Vec<u8>>>) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

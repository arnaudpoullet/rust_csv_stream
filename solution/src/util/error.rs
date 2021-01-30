use std::error::Error;
use std::string::FromUtf8Error;

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
    fn from(err: String) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

impl From<csv_async::Error> for ResponseError {
    fn from(err: csv_async::Error) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

impl From<std::io::Error> for ResponseError {
    fn from(err: std::io::Error) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

impl From<FromUtf8Error> for ResponseError {
    fn from(err: FromUtf8Error) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

impl From<reqwest::Error> for ResponseError {
    fn from(err: reqwest::Error) -> Self {
        ResponseError(format!("{:?}", err))
    }
}

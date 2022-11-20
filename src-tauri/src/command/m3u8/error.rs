use serde::ser::{Serialize, SerializeTupleVariant};
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum M3u8Error {
    HTTPError(reqwest::Error),
    HTTPCode(reqwest::StatusCode),
    Other,
}

// Allow the use of "{}" format specifier
impl Display for M3u8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            M3u8Error::HTTPError(e) => write!(f, "HTTPError by reqwest: {}", e.to_string()),
            M3u8Error::HTTPCode(c) => write!(f, "HTTP request reply with Code: {}", c),
            M3u8Error::Other => write!(f, "Unexpected Error, no info provided."),
        }
    }
}

// implemented for error conversion
impl From<reqwest::Error> for M3u8Error {
    fn from(cause: reqwest::Error) -> Self {
        M3u8Error::HTTPError(cause)
    }
}

// TODO:
impl Serialize for M3u8Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            M3u8Error::HTTPError(ref a) => {
                let mut tv = serializer.serialize_tuple_variant("M3u8Error", 0, "HTTPError", 1)?;
                // tv.serialize_field(a)?;
                tv.end()
            }
            M3u8Error::HTTPCode(ref a) => {
                let mut tv = serializer.serialize_tuple_variant("M3u8Error", 1, "HTTPCode", 1)?;
                // tv.serialize_field(a)?;
                tv.end()
            }
            M3u8Error::Other => {
                let mut tv = serializer.serialize_tuple_variant("M3u8Error", 2, "Other", 0)?;
                tv.end()
            }
        }
    }
}

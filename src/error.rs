// Copyright (c) 2016, Mikkel Kroman <mk@uplink.io>
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ApiError(String),
    JsonError(serde_json::Error),
    HttpError(reqwest::Error),
    HttpHeaderError(reqwest::header::ToStrError),
    HttpInvalidHeaderError(reqwest::header::InvalidHeaderValue),
    InvalidFilter(String),
    Io(io::Error),
    TrackNotDownloadable,
    TrackNotStreamable,
    UrlParseError(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::JsonError(ref error) => write!(f, "JSON error: {}", error),
            Error::HttpError(ref error) => write!(f, "HTTP error: {}", error),
            Error::HttpHeaderError(ref error) => write!(f, "HTTP error: {}", error),
            Error::HttpInvalidHeaderError(ref error) => write!(f, "HTTP error: {}", error),
            Error::ApiError(ref error) => write!(f, "SoundCloud error: {}", error),
            Error::Io(ref error) => write!(f, "IO error: {}", error),
            Error::InvalidFilter(_) => write!(f, "Invalid filter"),
            Error::TrackNotStreamable => write!(f, "The track is not available for streaming"),
            Error::TrackNotDownloadable => write!(f, "The track is not available for download"),
            Error::UrlParseError(ref error) => write!(f, "URL parsing error: {}", error),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::JsonError(ref error) => Some(error),
            Error::HttpError(ref error) => Some(error),
            Error::Io(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::HttpError(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::JsonError(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(error: reqwest::header::ToStrError) -> Error {
        Error::HttpHeaderError(error)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(error: reqwest::header::InvalidHeaderValue) -> Error {
        Error::HttpInvalidHeaderError(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Error {
        Error::UrlParseError(error)
    }
}

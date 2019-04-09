//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use std::{fmt, error};

#[derive(Debug, Clone)]
pub struct StorageError(pub String);

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for StorageError {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while parsing the server response")
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        "Error while parsing the server response"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionError;

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not connect to yocto server")
    }
}

impl error::Error for ConnectionError {
    fn description(&self) -> &'static str {
        "Could not connect to yocto server"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
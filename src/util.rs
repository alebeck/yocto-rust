//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use std::io::prelude::*;
use std::net::TcpStream;
use std::{result, str};
use crate::error::{StorageError, ParseError};

type Result<T> = result::Result<T, Box<std::error::Error>>;

pub const HEARTBEAT: &str = "HEARTBEAT";
pub const GET: &str = "GET";
pub const INSERT: &str = "INSERT";
pub const REMOVE: &str = "REMOVE";
pub const CLEAR: &str = "CLEAR";
pub const SEP: char = '\u{1f}';

pub fn send(request: &str, connection: &str) -> Result<String> {
    let mut stream = TcpStream::connect(connection)?;

    stream.write(request.as_bytes())?;
    stream.flush()?;

    let mut buffer = [0; 524288];
    stream.read(&mut buffer);

    let response = str::from_utf8(&buffer[..])?
        .trim_end_matches(char::from(0))
        .to_string();

    Ok(response)
}

pub fn parse(string: &str) -> Result<Option<String>> {
    let split: Vec<&str> = string.split(SEP).collect();

    let message = if split.len() > 1 {
        Some(split[1].to_string())
    } else {
        None
    };

    match split[0] {
        "OK" => Ok(message),
        "ERR" => Err(Box::new(StorageError(message.unwrap()))),
        _ => Err(Box::new(ParseError))
    }
}

pub fn handle(request: &str, connection: &str) -> Result<Option<String>> {
    let response = send(request, connection)?;
    parse(&response)
}
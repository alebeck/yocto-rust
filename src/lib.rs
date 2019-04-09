//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

pub mod error;
mod util;

use std::result;
use util::*;
use crate::error::ConnectionError;

type Result<T> = result::Result<T, Box<std::error::Error>>;

/// Represents a key-value store
pub struct Store {
    connection: String
}

impl Store {

    /// Returns a new yocto store
    ///
    /// # Arguments
    ///
    /// * `connection` - A string slice that holds the endpoint of the yocto server
    ///
    pub fn new(connection: &str) -> Result<Store> {
        // test connection
        let result = handle("TEST", connection);

        if let Err(e) = result {
            return Err(Box::new(ConnectionError));
        }

        Ok(Store {
            connection: connection.to_string()
        })
    }

    /// Locates the given key inside the database and returns an Ok with the
    /// corresponding value if existing or an None if not.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key to look up
    ///
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        handle(&format!("{}{}{}", GET, SEP, key), &self.connection)
    }

    /// Inserts a value for a specified key. Returns the old value if existent.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key to insert the value for
    /// * `value` - The value to insert
    ///
    pub fn insert(&self, key: &str, value: &str) -> Result<Option<String>>  {
        handle(&format!("{}{}{}{}{}", INSERT, SEP, key, SEP, value), &self.connection)
    }

    /// Removes the value corresponding to a key. Returns Err if key is not found.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the key to delete the value for
    ///
    pub fn remove(&self, key: &str) -> Result<Option<String>>  {
        handle(&format!("{}{}{}", REMOVE, SEP, key), &self.connection)
    }

    /// Removes all key-value pairs from the database
    ///
    pub fn clear(&self) -> Result<Option<String>>  {
        handle(&format!("{}", CLEAR), &self.connection)
    }
}
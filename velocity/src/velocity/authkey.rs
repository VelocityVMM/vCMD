//! Authkey definition and utility implementations
use std::{
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    error::{ClientError, VelocityError},
    Velocity,
};

use wasm_bindgen::prelude::wasm_bindgen;

/// An authkey has a key value that authenticates and an expiration datetime
#[derive(Debug, Clone)]
#[allow(dead_code)]
#[wasm_bindgen]
pub struct Authkey {
    /// The key string to authenticate to Velocity
    key: String,
    /// The UNIX timestamp of the expiration date
    expires: SystemTime,
}

impl Authkey {
    /// Creates a new authkey
    /// # Arguments
    /// * `key` - The key value to use
    /// * `expires` - The time since the UNIX epoch in seconds the key expires
    pub fn new(key: &str, expires: u64) -> Self {
        Self {
            key: key.to_string(),
            expires: UNIX_EPOCH.add(Duration::from_secs(expires)),
        }
    }

    /// Returns a `&str` to the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns a reference to the expiration time
    pub fn expires(&self) -> &SystemTime {
        &self.expires
    }
}

#[wasm_bindgen]
impl Velocity {
    /// Tries to retrieve the authkey from this instance. If it doesn't exist, this will error
    /// with a `ClientError::NotAuthenticated`
    pub fn get_authkey(&self) -> Result<Authkey, VelocityError> {
        match &self.authkey {
            Some(key) => Ok(key.clone()),
            None => Err(VelocityError::Client(ClientError::NotAuthenticated)),
        }
    }
}

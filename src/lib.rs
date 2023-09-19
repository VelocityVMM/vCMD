//! VCMD is an API wrapper for the [Velocity](https://github.com/VelocityVMM/Velocity) hypervisor API. This crate allows easy access and bundling to all those endpoints and wraps them up nicely in pure Rust.
//! # Example
//! ```
//! use vcmd::Velocity;
//!
//! // Create a new Velocity instance, this will later be used to access other endpoints.
//! // We immediately authenticate to get a valid connection to the hypervisor.
//! let mut velocity = Velocity::new("http://localhost:8090", "root", "root").unwrap();
//!
//! // Reauthenticate this instance. This has to be done every once in a while to not loose access
//! velocity.reauthenticate().unwrap();
//!
//! // We're done here, so let's deauthenticate and say goodbye to the hypervisor
//! velocity.deauthenticate().unwrap();
//! ```

mod velocity;
use error::*;
pub use velocity::{authkey::*, *};

/// This struct is the main workhorse of this API client, all requests and functions go through
/// this struct and its methods
#[derive(Debug)]
pub struct Velocity<'a> {
    base_url: &'a str,
    http_client: reqwest::blocking::Client,
    authkey: Option<Authkey>,
}

impl<'a> Velocity<'a> {
    /// Creates a new and authenticated `Velocity` instance. If the authentication fails, this will error out
    /// # Arguments
    /// * `base_url` - The base url to route all requests to
    /// * `username` - The username needed for authentication
    /// * `password` - The passwrod needed for authentication
    pub fn new(base_url: &'a str, username: &str, password: &str) -> Result<Self, VelocityError> {
        let http_client = reqwest::blocking::Client::new();
        let mut v = Velocity {
            base_url,
            http_client,
            authkey: Default::default(),
        };

        v.authenticate(username, password)?;

        Ok(v)
    }
}

impl<'a> Drop for Velocity<'a> {
    /// Tries to deauthenticate this client before dropping it
    fn drop(&mut self) {
        if self.authkey.is_some() {
            // Try to deauthenticate, if this fails we don't care since the key
            // will eventually expire and the hypervisor will catch that
            drop(self.deauthenticate());
        }
    }
}

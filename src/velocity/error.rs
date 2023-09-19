//! Errors
use std::{error::Error, fmt::Display};

use serde::Deserialize;

/// An enumeration of all possible errors that can result from this API
#[derive(Debug)]
pub enum VelocityError {
    /// An API error, indicates that there is some error of usage or request
    APIError(VelocityAPIError),
    /// An error thrown by the reqwest crate, regarding client-side errors
    Reqwest(reqwest::Error),
    /// An error by the client / user of this library
    Client(ClientError),
}

/// A ClientError is an error that regards the user of this API
#[derive(Debug)]
pub enum ClientError {
    /// An authkey is required, but there is none
    NotAuthenticated,
}

impl ClientError {
    pub fn message(&self) -> String {
        match self {
            Self::NotAuthenticated => "This client is not authenticated",
        }
        .to_owned()
    }
}

/// A Velocity API error's basic structure: All errors get transmitted in this format
#[derive(Debug, Deserialize)]
pub struct VelocityAPIError {
    /// The code for the error. See the `Errors.md` article in the hypervisor API docs
    pub code: u32,
    /// A message describing the error in a human-readable format
    pub message: String,
}

impl From<reqwest::Error> for VelocityError {
    fn from(value: reqwest::Error) -> Self {
        VelocityError::Reqwest(value)
    }
}

impl Display for VelocityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => e.fmt(f),
            Self::APIError(e) => write!(f, "{}: {}", e.code, e.message),
            Self::Client(e) => match e {
                ClientError::NotAuthenticated => write!(f, "{}", e.message()),
            },
        }
    }
}

impl Error for VelocityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

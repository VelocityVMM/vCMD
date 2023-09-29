use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{Authkey, Velocity, VelocityError};

impl Velocity {
    /// Authenticate this Velocity instance
    /// # Arguments
    /// * `username` - The username for the user
    /// * `password` - The password for the user
    /// # Returns
    /// The key on success
    pub async fn authenticate(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<Authkey, VelocityError> {
        let req = UAuthPOSTReq { username, password };

        let res = self
            .request_json::<UAuthPOSTReq, UAuthPOSTRes>(Method::POST, "/u/auth", &req)
            .await?;

        let key = Authkey::new(&res.response.authkey, res.response.expires);

        self.authkey = Some(key.clone());

        Ok(key)
    }

    /// Reauthenticates the current authkey
    /// # Returns
    /// The key on success
    pub async fn reauthenticate(&mut self) -> Result<Authkey, VelocityError> {
        let authkey = self.get_authkey()?;

        let req = UAuthPATCHReq {
            authkey: authkey.key(),
        };

        let res = self
            .request_json::<UAuthPATCHReq, UAuthPATCHRes>(Method::PATCH, "/u/auth", &req)
            .await?;

        let key = Authkey::new(&res.response.authkey, res.response.expires);

        self.authkey = Some(key.clone());

        Ok(key)
    }

    /// Deauthenticates and drops the internal authkey
    pub async fn deauthenticate(&mut self) -> Result<(), VelocityError> {
        let authkey = self.get_authkey()?;

        let req = UAuthDELETEReq {
            authkey: authkey.key(),
        };

        let res = self.request(Method::DELETE, "/u/auth", &req).await?;

        if res == StatusCode::OK {
            self.authkey = None
        }

        Ok(())
    }
}

/// `/u/auth - POST` Request structure
#[derive(Serialize)]
struct UAuthPOSTReq<'a> {
    username: &'a str,
    password: &'a str,
}

/// `/u/auth - PATCH` Response structure
#[derive(Deserialize)]
struct UAuthPOSTRes {
    authkey: String,
    expires: u64,
}

/// `/u/auth - DELETE` Request structure
#[derive(Serialize)]
struct UAuthDELETEReq<'a> {
    authkey: &'a str,
}

/// `/u/auth - PATCH` Request structure
#[derive(Serialize)]
struct UAuthPATCHReq<'a> {
    authkey: &'a str,
}

/// `/u/auth - PATCH` Response structure
#[derive(Deserialize)]
struct UAuthPATCHRes {
    authkey: String,
    expires: u64,
}

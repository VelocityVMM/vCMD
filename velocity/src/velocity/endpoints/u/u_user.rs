use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{error::VelocityError, Velocity};

use crate::{GID, PID, UID};

impl Velocity {
    /// Creates a new user using the provided credentials
    /// # Arguments
    /// * `username` - The unique username to use for the newly created user
    /// * `password` - The password to use for the newly created user
    /// # Return
    /// The `uid` of the new user
    pub async fn user_create(&self, username: &str, password: &str) -> Result<UID, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UUserPUTReq {
            authkey: authkey.key(),
            name: username,
            password,
        };

        Ok(self
            .request_json::<UUserPUTReq, UUserPUTRes>(Method::PUT, "/u/user", &request)
            .await?
            .response
            .uid)
    }

    /// Removes a user with the supplied user id
    /// # Arguments
    /// * `uid` - The `uid` of the user to remove
    pub async fn user_remove(&self, uid: UID) -> Result<(), VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UUserDELETEReq {
            authkey: authkey.key(),
            uid,
        };

        self.request(Method::DELETE, "/u/user", &request).await?;

        Ok(())
    }

    /// Retrieves user information about the user
    /// # Arguments
    /// * `uid` - The `uid`, or None for the user that is authenticated by the current authkey
    pub async fn user_info(&self, uid: Option<UID>) -> Result<UserInfo, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UUserPOSTReq {
            authkey: authkey.key(),
            uid,
        };

        Ok(self
            .request_json::<UUserPOSTReq, UserInfo>(Method::POST, "/u/user", &request)
            .await?
            .response)
    }

    /// Lists all users on a velocity instance
    pub async fn user_list(&self) -> Result<Vec<UUserListPOSTRes>, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UUserListPOSTReq {
            authkey: authkey.key(),
        };

        #[derive(Deserialize)]
        struct Res {
            users: Vec<UUserListPOSTRes>,
        }

        Ok(self
            .request_json::<UUserListPOSTReq, Res>(Method::POST, "/u/user/list", &request)
            .await?
            .response
            .users)
    }
}

/// A struct providing information about a user
#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub uid: UID,
    pub name: String,
    pub memberships: Vec<Membership>,
}

impl Display for UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Memberships for user '{}' ({})", self.name, self.uid)?;

        for membership in &self.memberships {
            writeln!(f, " - Group '{}' ({}):", membership.name, membership.gid)?;
            for permission in &membership.permissions {
                writeln!(
                    f,
                    "   |- Permission [{:<2}] '{}'",
                    permission.pid, permission.name,
                )?;
            }
        }

        Ok(())
    }
}

/// Describes a membership
#[derive(Deserialize, Debug)]
pub struct Membership {
    pub gid: GID,
    pub parent_gid: GID,
    pub name: String,
    pub permissions: Vec<Permission>,
}

/// A permission
#[derive(Deserialize, Debug)]
pub struct Permission {
    pub pid: PID,
    pub name: String,
    pub description: String,
}

/// `/u/user - POST` Request structure
#[derive(Serialize)]
struct UUserPOSTReq<'a> {
    authkey: &'a str,
    uid: Option<UID>,
}

/// `/u/user - PUT` Request structure
#[derive(Serialize)]
struct UUserPUTReq<'a> {
    authkey: &'a str,
    name: &'a str,
    password: &'a str,
}

/// `/u/user - PUT` Response structure
#[derive(Deserialize, Debug)]
pub struct UUserPUTRes {
    pub uid: UID,
    pub name: String,
}

/// `/u/user - DELETE` Request structure
#[derive(Serialize)]
struct UUserDELETEReq<'a> {
    authkey: &'a str,
    uid: UID,
}

/// `/u/user/list - POST` Request structure
#[derive(Serialize, Debug)]
struct UUserListPOSTReq<'a> {
    authkey: &'a str,
}

/// `/u/user/list - POST` Response structure
#[derive(Deserialize, Debug)]
pub struct UUserListPOSTRes {
    pub uid: UID,
    pub name: String,
}

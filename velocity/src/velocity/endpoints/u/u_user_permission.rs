use reqwest::Method;
use serde::Serialize;

use crate::{Velocity, VelocityError, GID, UID};

impl Velocity {
    /// Adds a new permission to a user on a group
    /// # Arguments
    /// * `gid` - The group to permit on
    /// * `uid` - The user to permit
    /// * `permission` - The permission string to permit
    pub async fn user_add_permission(
        &self,
        gid: GID,
        uid: UID,
        permission: &str,
    ) -> Result<(), VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UUserPermissionPUTReq {
            authkey: authkey.key(),
            gid,
            uid,
            permission,
        };

        self.request(Method::PUT, "/u/user/permission", &request)
            .await?;

        Ok(())
    }

    /// Revoke a permission of a user on a group
    /// # Arguments
    /// * `gid` - The group to revoke from
    /// * `uid` - The user to revoke the permission from
    /// * `permission` - The permission string to revoke
    pub async fn user_revoke_permission(
        &self,
        gid: GID,
        uid: UID,
        permission: &str,
    ) -> Result<(), VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UUserPermissionDELETEReq {
            authkey: authkey.key(),
            gid,
            uid,
            permission,
        };

        self.request(Method::DELETE, "/u/user/permission", &request)
            .await?;

        Ok(())
    }
}

/// `/u/user/permission - PUT` Request structure
#[derive(Serialize)]
struct UUserPermissionPUTReq<'a> {
    authkey: &'a str,
    gid: GID,
    uid: UID,
    permission: &'a str,
}

/// `/u/user/permission - DELETE` Request structure
#[derive(Serialize)]
struct UUserPermissionDELETEReq<'a> {
    authkey: &'a str,
    gid: GID,
    uid: UID,
    permission: &'a str,
}

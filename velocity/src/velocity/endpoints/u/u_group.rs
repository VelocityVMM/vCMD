use std::fmt::Display;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{error::VelocityError, Velocity};

use crate::{GID, UID};

use super::u_user::Permission;

impl Velocity {
    /// Provides group information
    /// # Arguments
    /// * `gid` - The group id of the group to inform about
    pub async fn group_info(&self, gid: GID) -> Result<GroupInfo, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UGroupPOSTReq {
            authkey: authkey.key(),
            gid,
        };

        Ok(self
            .request_json::<UGroupPOSTReq, GroupInfo>(Method::POST, "/u/group", &request)
            .await?
            .response)
    }

    /// Creates a new group within the Velocity system
    /// # Arguments
    /// * `parent_gid` - The `gid` of the parent group this new group should be a part of
    /// * `name` - A unique name for the new group within the parent group
    pub async fn group_create(
        &self,
        parent_gid: GID,
        name: &str,
    ) -> Result<UGroupPUTRes, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UGroupPUTReq {
            authkey: authkey.key(),
            parent_gid,
            name,
        };

        Ok(self
            .request_json::<UGroupPUTReq, UGroupPUTRes>(Method::PUT, "/u/group", &request)
            .await?
            .response)
    }

    /// Removes a group from the Velocity system
    /// # Arguments
    /// * `gid` - The `gid` of the group to remove
    pub async fn group_remove(&self, gid: GID) -> Result<(), VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UGroupDELETEReq {
            authkey: authkey.key(),
            gid,
        };

        self.request(Method::DELETE, "/u/group", &request).await?;

        Ok(())
    }

    /// List all groups visible to the current user
    pub async fn group_list(&self) -> Result<Vec<UGroupListPOSTRes>, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = UGroupListPOSTReq {
            authkey: authkey.key(),
        };

        #[derive(Deserialize, Debug)]
        struct Res {
            groups: Vec<UGroupListPOSTRes>,
        }

        Ok(self
            .request_json::<UGroupListPOSTReq, Res>(Method::POST, "/u/group/list", &request)
            .await?
            .response
            .groups)
    }
}

/// A struct providing information about a group
#[derive(Deserialize, Debug)]
pub struct GroupInfo {
    pub name: String,
    pub gid: GID,
    pub parent_gid: GID,
    pub memberships: Vec<GroupMembership>,
}

impl Display for GroupInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Memberships for group '{}' ({}, parent: {})",
            self.name, self.gid, self.parent_gid
        )?;

        for membership in &self.memberships {
            writeln!(f, " - User '{}' ({}):", membership.name, membership.uid)?;
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
pub struct GroupMembership {
    pub uid: UID,
    pub name: String,
    pub permissions: Vec<Permission>,
}

/// `/u/group - POST` Request structure
#[derive(Serialize)]
struct UGroupPOSTReq<'a> {
    authkey: &'a str,
    gid: GID,
}

/// `/u/group - PUT` Request structure
#[derive(Serialize)]
struct UGroupPUTReq<'a> {
    authkey: &'a str,
    name: &'a str,
    parent_gid: GID,
}

/// `/u/group - PUT` Response structure
#[derive(Deserialize, Debug)]
pub struct UGroupPUTRes {
    pub gid: GID,
    pub parent_gid: GID,
    pub name: String,
}

/// `/u/group - DELETE` Request structure
#[derive(Serialize)]
struct UGroupDELETEReq<'a> {
    authkey: &'a str,
    gid: GID,
}

/// `/u/group/list - POST` Request structure
#[derive(Serialize)]
struct UGroupListPOSTReq<'a> {
    authkey: &'a str,
}

/// `/u/group/list - POST` Response structure
#[derive(Deserialize, Debug)]
pub struct UGroupListPOSTRes {
    pub name: String,
    pub gid: GID,
    pub parent_gid: GID,
    pub permissions: Vec<Permission>,
}

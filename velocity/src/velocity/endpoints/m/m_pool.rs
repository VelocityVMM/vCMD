use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{error::VelocityError, Velocity, GID, MPID};

impl Velocity {
    /// List all pools assigned to the group with the provided `gid`
    ///
    /// To retrieve a list of all pools available, run this command against the
    /// `root` group (GID=0)
    /// # Arguments
    /// * `gid` - The group id of the group to look up
    pub async fn pool_list(&self, gid: GID) -> Result<Vec<MPoolListPOSTRes>, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = MPoolListPOSTReq {
            authkey: authkey.key(),
            gid,
        };

        #[derive(Deserialize, Debug)]
        struct Res {
            pools: Vec<MPoolListPOSTRes>,
        }

        Ok(self
            .request_json::<MPoolListPOSTReq, Res>(Method::POST, "/m/pool/list", &request)
            .await?
            .response
            .pools)
    }

    /// Assign a pool to a group
    /// # Arguments
    /// * `gid` - The group id of the group to assign to
    /// * `mpid` - The mediapool id of the pool to assign
    /// * `quota` - The quota in bytes for the assignment
    /// * `write` - If the group can write to pool media
    /// * `manage` - If the group can create, delete media and manage the pool
    pub async fn pool_assign(
        &self,
        gid: GID,
        mpid: MPID,
        quota: u64,
        write: bool,
        manage: bool,
    ) -> Result<(), VelocityError> {
        let authkey = self.get_authkey()?;

        let request = MPoolAssignPUTReq {
            authkey: authkey.key(),
            gid,
            mpid,
            quota,
            write,
            manage,
        };

        self.request(Method::PUT, "/m/pool/assign", &request)
            .await?;

        Ok(())
    }

    /// Revoke all permissions of a group on a mediapool
    /// # Arguments
    /// * `gid` - The group id of the group to revoke from
    /// * `mpid` - The mediapool id of the pool to revoke
    pub async fn pool_revoke(&self, gid: GID, mpid: MPID) -> Result<(), VelocityError> {
        let authkey = self.get_authkey()?;

        let request = MPoolAssignDELETEReq {
            authkey: authkey.key(),
            gid,
            mpid,
        };

        self.request(Method::DELETE, "/m/pool/assign", &request)
            .await?;

        Ok(())
    }
}

/// `/m/pool/list - POST` Request structure
#[derive(Serialize)]
struct MPoolListPOSTReq<'a> {
    authkey: &'a str,
    gid: GID,
}

/// `/m/pool/list - POST` Response structure
#[derive(Deserialize, Debug)]
pub struct MPoolListPOSTRes {
    pub mpid: MPID,
    pub name: String,
    pub write: bool,
    pub manage: bool,
}

/// `/m/pool/assign - PUT` Request structure
#[derive(Serialize)]
struct MPoolAssignPUTReq<'a> {
    authkey: &'a str,
    gid: GID,
    mpid: MPID,
    quota: u64,
    write: bool,
    manage: bool,
}

/// `/m/pool/assign - PUT` Response structure
#[derive(Serialize)]
struct MPoolAssignDELETEReq<'a> {
    authkey: &'a str,
    gid: GID,
    mpid: MPID,
}

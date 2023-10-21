use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{error::VelocityError, Velocity, GID, MID, MPID};

impl Velocity {
    /// List all available media for a group
    /// # Arguments
    /// * `gid` - The group id of the group to list of
    pub async fn media_list(&self, gid: GID) -> Result<Vec<MMediaListPOSTRes>, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = MMediaListPOSTReq {
            authkey: authkey.key(),
            gid,
        };

        #[derive(Deserialize, Debug)]
        struct Res {
            media: Vec<MMediaListPOSTRes>,
        }

        Ok(self
            .request_json::<MMediaListPOSTReq, Res>(Method::POST, "/m/media/list", &request)
            .await?
            .response
            .media)
    }

    /// Allocate new media on the hypervisor
    /// # Arguments
    /// * `mpid` - The mediapool id where the media should live in
    /// * `gid` - The group id that should own the newly created media
    /// * `name` - A user-friendly name for the new media
    /// * `ty` - A string describing the type of media to use - see the Velocity API documentation
    /// * `size` - The size in bytes for the new media
    pub async fn media_allocate(
        &self,
        mpid: MPID,
        gid: GID,
        name: &str,
        ty: &str,
        size: u64,
    ) -> Result<MMediaCreatePUTRes, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = MMediaCreatePUTReq {
            authkey: authkey.key(),
            mpid,
            gid,
            name,
            ty,
            size,
        };

        Ok(self
            .request_json(Method::PUT, "/m/media/create", &request)
            .await?
            .response)
    }

    /// Remove a piece of media and delete it
    /// # Arguments
    /// * `mid` - The media id of the media to remove
    pub async fn media_remove(&self, mid: MID) -> Result<(), VelocityError> {
        let authkey = self.get_authkey()?;

        let request = MMediaDELETEReq {
            authkey: authkey.key(),
            mid,
        };

        self.request(Method::DELETE, "/m/media", &request).await?;

        Ok(())
    }
}

/// `/m/media/list - POST` Request structure
#[derive(Serialize)]
struct MMediaListPOSTReq<'a> {
    authkey: &'a str,
    gid: GID,
}

/// `/m/media/list - POST` Response structure
#[derive(Deserialize, Debug)]
pub struct MMediaListPOSTRes {
    pub mid: MID,
    pub mpid: MPID,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub size: u64,
    pub readonly: bool,
}

/// `/m/media/create - PUT` Request structure
#[derive(Serialize)]
struct MMediaCreatePUTReq<'a> {
    authkey: &'a str,
    mpid: MPID,
    gid: GID,
    name: &'a str,
    #[serde(rename = "type")]
    ty: &'a str,
    size: u64,
}

/// `/m/media/create - PUT` Response structure
#[derive(Deserialize, Debug)]
pub struct MMediaCreatePUTRes {
    pub mid: MID,
    pub size: u64,
}

/// `/m/media - DELETE` Request structure
#[derive(Serialize)]
struct MMediaDELETEReq<'a> {
    authkey: &'a str,
    mid: MID,
}

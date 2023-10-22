use futures_util::StreamExt;
use reqwest::{header::HeaderMap, Body, Method};
use serde::{Deserialize, Serialize};
use tokio::fs::File;

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

    /// Upload a file to the hypervisor as a new piece of media
    /// # Arguments
    /// * `mpid` - The mediapool id where the media should live in
    /// * `gid` - The group id that should own the newly created media
    /// * `name` - A user-friendly name for the new media
    /// * `ty` - A string describing the type of media to use - see the Velocity API documentation
    /// * `readonly` - If the file should be read-only
    /// * `file` - The file to upload
    /// * `progress_callback` - A callback informing the caller about the upload progress. Args: `(total: u64, uploaded: u64)`
    pub async fn media_upload<F>(
        &self,
        mpid: MPID,
        gid: GID,
        name: &str,
        ty: &str,
        readonly: bool,
        file: File,
        progress_callback: F,
    ) -> Result<MMediaUploadPUTRes, VelocityError>
    where
        F: Fn(u64, u64) + Send + Sync + 'static,
    {
        let authkey = self.get_authkey()?;

        let mut headers = HeaderMap::new();
        let file_size = file.metadata().await.expect("Read File metadata").len();

        // Apply all headers for the Velocity API
        // This API call does not have a JSON request, but relies on HTTP headers
        headers.insert("Content-Length", file_size.into());
        headers.insert(
            "x-velocity-authkey",
            authkey.key().parse().expect("Parse authkey to HeaderValue"),
        );
        headers.insert("x-velocity-mpid", mpid.into());
        headers.insert("x-velocity-gid", gid.into());
        headers.insert(
            "x-velocity-name",
            name.parse().expect("Parse name to HeaderValue"),
        );
        headers.insert(
            "x-velocity-type",
            ty.parse().expect("Parse type to HeaderValue"),
        );
        headers.insert(
            "x-velocity-readonly",
            match readonly {
                true => "true",
                false => "false",
            }
            .parse()
            .unwrap(),
        );

        // Create a ReaderStream to provide a way of obtaining progress
        let mut reader_stream = tokio_util::io::ReaderStream::new(file);

        // The amount of bytes uploaded until now
        let mut uploaded = 0u64;

        // Move the chunks from the file into the stream, noting down progress
        let async_stream = async_stream::stream! {
            while let Some(chunk) = reader_stream.next().await {
                if let Ok(chunk) = &chunk {
                    // Append the newly uploaded bytes and call the progress callback
                    uploaded += chunk.len() as u64;
                    progress_callback(file_size, uploaded);
                }
                yield chunk;
            }
        };

        // Construct the request
        let request = self
            .http_client
            .put(self.url("/m/media/upload"))
            .headers(headers)
            .body(Body::wrap_stream(async_stream));

        // And expect the response
        Ok(self
            .request_json_raw::<MMediaUploadPUTRes>(request)
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

/// `/m/media/upload - PUT` Response structure
#[derive(Deserialize, Debug)]
pub struct MMediaUploadPUTRes {
    pub mid: MID,
    pub size: u64,
}

/// `/m/media - DELETE` Request structure
#[derive(Serialize)]
struct MMediaDELETEReq<'a> {
    authkey: &'a str,
    mid: MID,
}

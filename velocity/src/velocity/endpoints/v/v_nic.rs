use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{error::VelocityError, Velocity, NICID};

impl Velocity {
    /// List all available host NICs to user for bridge mode
    pub async fn nic_list(&self) -> Result<Vec<VNICListPOSTRes>, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = VNICListPOSTReq {
            authkey: authkey.key(),
        };

        #[derive(Deserialize, Debug)]
        struct Res {
            host_nics: Vec<VNICListPOSTRes>,
        }

        Ok(self
            .request_json::<VNICListPOSTReq, Res>(Method::POST, "/v/nic/list", &request)
            .await?
            .response
            .host_nics)
    }
}

/// `/v/nic/list - POST` Request structure
#[derive(Serialize)]
struct VNICListPOSTReq<'a> {
    authkey: &'a str,
}

/// `/v/nic/list - POST` Response structure
#[derive(Deserialize, Debug)]
pub struct VNICListPOSTRes {
    pub nicid: NICID,
    pub description: String,
    pub identifier: String,
}

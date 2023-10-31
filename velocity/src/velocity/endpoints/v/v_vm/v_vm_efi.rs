use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{error::VelocityError, Velocity, VMID};

use super::CoreVM;

impl Velocity {
    /// Create a new EFI virtual machine
    /// # Arguments
    /// * `vm_config` - A config that describes the virtual machine
    pub async fn vm_efi_create<'a>(
        &self,
        vm_config: EFIVMConfig<'a>,
    ) -> Result<VMID, VelocityError> {
        let authkey = self.get_authkey()?;

        let request = VVMEFIPUTReq {
            authkey: authkey.key(),
            config: vm_config,
        };

        let vmid = self
            .request_json::<VVMEFIPUTReq, VVMEFIPUTRes>(Method::PUT, "/v/vm/efi", &request)
            .await?
            .response
            .vmid;

        Ok(vmid)
    }
}

/// A configuration for a `EFI` virtual machine
#[derive(Serialize)]
pub struct EFIVMConfig<'a> {
    /// The core virtual machine configuration
    #[serde(flatten)]
    pub core: CoreVM<'a>,

    /// If this virtual machine should use the `rosetta` translation layer if available
    pub rosetta: bool,
}

/// `/v/vm/efi - PUT` Request structure
#[derive(Serialize)]
struct VVMEFIPUTReq<'a> {
    authkey: &'a str,
    #[serde(flatten)]
    config: EFIVMConfig<'a>,
}

/// `/v/vm/efi - PUT` Response structure
#[derive(Debug, Deserialize)]
struct VVMEFIPUTRes {
    vmid: VMID,
}

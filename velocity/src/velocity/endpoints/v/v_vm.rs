use std::str::FromStr;

use crate::{GID, MID, NICID};
use serde::Serialize;

pub mod v_vm_efi;

/// A core virtual machine configuration that is common across
/// all virtual machine types
#[derive(Debug, Serialize)]
pub struct CoreVM<'a> {
    /// The name for the virtual machine
    pub name: &'a str,
    /// The group id the virtual machine should belong to
    pub gid: GID,

    /// The amount of CPUs the virtual machine should be allowed to use
    pub cpus: u32,
    /// The amount of memory
    pub memory: u64,

    pub displays: Vec<DisplayConfig>,
    pub disks: Vec<DiskConfig>,
    pub nics: Vec<NICConfig>,

    pub autostart: bool,
}

/// A configuration for a display for a virtual machine
#[derive(Debug, Serialize)]
pub struct DisplayConfig {
    /// A user-friendly name for the display
    pub name: String,
    /// The width in pixels
    pub width: u32,
    /// The height in pixels
    pub height: u32,
    /// The pixels per inch value
    pub ppi: u32,
}

/// A configuration for a disk to attach to a virtual machine
#[derive(Debug, Serialize)]
pub struct DiskConfig {
    /// The media id of the piece of media to attach
    pub mid: MID,
    /// The mode to use for the disk
    pub mode: DiskMode,
    /// If the disk should be read-only
    pub readonly: bool,
}

/// The possible modes a disk can be attached to a virtual machine
#[derive(Debug, Serialize)]
pub enum DiskMode {
    /// Attach the disk over `USB`
    USB,
    /// Attach the disk using a block device
    BLOCK,
    /// Attach the disk over `VIRTIO`
    VIRTIO,
}

/// A configuration for a virtual machine NIC
#[derive(Debug, Serialize)]
pub struct NICConfig {
    /// The type of NIC to use
    #[serde(rename = "type")]
    pub ty: NICType,
    /// If `ty` is `BRIDGE`, supply the host `NICID`
    pub host: Option<NICID>,
}

/// The possible types a NIC can be in a virtual machine
#[derive(Debug, Serialize)]
pub enum NICType {
    /// A `NAT` NIC
    NAT,
    /// A `BRIDGE` NIC, requiring a host NIC
    BRIDGE,
}

impl FromStr for DiskMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USB" => Ok(Self::USB),
            "BLOCK" => Ok(Self::BLOCK),
            "VIRTIO" => Ok(Self::VIRTIO),
            _ => Err("Available modes: 'USB', 'BLOCK', 'VIRTIO'".to_owned()),
        }
    }
}

impl FromStr for NICType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NAT" => Ok(Self::NAT),
            "BRIDGE" => Ok(Self::BRIDGE),
            _ => Err("Available modes: 'NAT', 'BRIDGE'".to_owned()),
        }
    }
}

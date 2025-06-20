use tea_arch_chroot_lib::resource::{FirmwareKind, MethodKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct Partition
{
    pub number: u64,
    pub disk_path: Option<String>,
    pub path: Option<String>,
    pub mountpoint: Option<String>,
    pub filesystem: Option<String>,
    pub label: Option<String>,
    pub format: bool,
    pub start: u64,
    pub end: u64,
    pub size: u64,
    pub flags: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OriginalSector {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct Storage
{
    pub original_sector: Option<OriginalSector>,
    pub disk_path: Option<String>,
    pub partition_table: Option<String>,
    pub new_partition_table: bool,
    pub layout_changed: bool,
    pub autogenerated: bool,
    pub autogenerated_mode: String, // this is will be deprecated soon and removed.
    pub partitions: Option<Vec<Partition>>,
    pub install_method: MethodKind
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bootloader {
    pub firmware_type: FirmwareKind,
    pub path: Option<String>,
}

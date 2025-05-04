// this file is executed when user want Erase disk & clean install
use duct::cmd;
use tea_arch_chroot_lib::resource::FirmwareKind;
use std::{clone, str::FromStr};
use serde::{Deserialize, Serialize};
use crate::blueprint::Storage;
use crate::exception;
use crate::disk_helper::{gb2sector, mb2sector};
use std::path::Path;


#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub disk: Disk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
    pub path: Option<String>,
    pub size: Option<String>,
    pub model: Option<String>,
    pub transport: Option<String>,
    #[serde(rename = "logical-sector-size")]
    pub logical_sector_size: u32,
    #[serde(rename = "physical-sector-size")]
    pub physical_sector_size: u32,
    pub label: Option<String>,
    pub uuid: Option<String>,
    #[serde(rename = "max-partitions")]
    pub max_partitions: u32,
    pub partitions: Vec<Partition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Partition {
    pub number: u32,
    pub start: String,
    pub end: String,
    pub size: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "type-uuid")]
    pub type_uuid: Option<String>,
    #[serde(rename = "type-id")]
    pub type_id: Option<String>,
    pub uuid: Option<String>,
    pub filesystem: String,
    pub flags: Option<Vec<String>>
}


#[derive(Debug, Clone)]
pub struct DualbootBlkstuff {
    pub selected_blockdev: String,
    pub selected_fs: String
}

#[derive(Default, Debug)]
pub struct DiskLayout {
    mode: Option<FirmwareKind>,
    partition_type: Option<String>
}

pub trait DualBootBlockdevice {
    fn blockdevice(blkname: String, fs: String) -> Self;
    fn check_base_disk_layout(&self) -> DiskLayout;
    fn parted_partition_structure(&self) -> Option<DiskInfo>;
    fn getresult(&self) -> Result<Storage, String>;
    fn _check(&self) -> Result<bool, String>;
}

impl DualBootBlockdevice for DualbootBlkstuff {
    fn blockdevice(blkname: String, fs: String) -> Self {
        DualbootBlkstuff {
            selected_blockdev: blkname,
            selected_fs: fs
        }
    }

    fn parted_partition_structure(&self) -> Option<DiskInfo> {
        let data = cmd!("parted", self.selected_blockdev.clone(), "-j", "print").read();

        if let Ok(data_val) = data {
            let parted_json = serde_json::from_str::<DiskInfo>(&data_val);

            match parted_json {
                Ok(val) => Some(val),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        } else {
            None
        }

    }

    fn check_base_disk_layout(&self) -> DiskLayout {
        let efi_firmware_path = Path::new("/sys/firmware/efi");
        let mut buf: DiskLayout = DiskLayout::default();

        if efi_firmware_path.exists() && efi_firmware_path.is_dir() {
            buf.mode = Some(FirmwareKind::UEFI);
        } else {
            buf.mode = Some(FirmwareKind::BIOS);
        }

        let check_disk_layout = self.parted_partition_structure();
        // println!("{:#?}", check_disk_layout);
        if let Some(check_disk_layout_val) = check_disk_layout {
            buf.partition_type = Some(check_disk_layout_val.disk.label.unwrap());
        } else {
            buf.partition_type = None;
        }

        buf
    }

    fn _check(&self) -> Result<bool, String> {
        let data = self.check_base_disk_layout();

        if let Some(mode_val) = data.mode {
            if data.partition_type == Some("gpt".to_string()) && mode_val == FirmwareKind::UEFI {
                return Ok(true);
            } 
            
            if data.partition_type == Some("msdos".to_string()) && mode_val == FirmwareKind::BIOS {
                return Ok(true);
            } 
        };

        return Err("partition and boot mode in yout system is mismatch or unusual".to_string());
        
    }

    fn getresult(&self) -> Result<Storage, String> {
        let check = self._check();

        if let Ok(check_val) = check {
            println!("{:#?}", check_val);
            Ok(Storage::default())
        } else {
            Err("generation failed".to_string())
        }

    }
}
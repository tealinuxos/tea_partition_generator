// this file is executed when user want Erase disk & clean install
use duct::cmd;
use tea_arch_chroot_lib::resource::{FirmwareKind, MethodKind};
use std::{clone, str::FromStr};
use serde::{Deserialize, Serialize};
use crate::blueprint::Storage;
// use crate::blueprint::{Storage, Partition};
use crate::exception;
use crate::disk_helper::{gb2sector, mb2sector};
use std::path::Path;
use crate::core::{PartitionGenerator, TeaPartitionGenerator};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    fn getresult(&self, start: u64, end: u64) -> Result<Storage, String>;
    fn _check(&self) -> Result<bool, String>;
    fn _generate_json(&self, start: u64, end: u64) -> Storage;
    // fn _disk_check_requirements(&self) -> Result<bool, String>;
    fn get_highest_partition_number(&self, data: &Option<DiskInfo>) -> i32;
}

impl DualBootBlockdevice for DualbootBlkstuff {
    fn blockdevice(blkname: String, fs: String) -> Self {
        DualbootBlkstuff {
            selected_blockdev: blkname,
            selected_fs: fs
        }
    }

    fn parted_partition_structure(&self) -> Option<DiskInfo> {
        let data = cmd!("parted", self.selected_blockdev.clone(), "-j", "unit", "s", "print").read();

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

        if let Some(ref mode_val) = data.mode {
            if data.partition_type == Some("gpt".to_string()) && *mode_val == FirmwareKind::UEFI {
                return Ok(true);
            } 
            
            if data.partition_type == Some("msdos".to_string()) && *mode_val == FirmwareKind::BIOS {
                return Ok(true);
            } 
        };

        let buf = format!("your partition (which is {}) and boot mode (which is {}) in yout system is mismatch or unusual", data.partition_type.unwrap().to_uppercase(), data.mode.unwrap().as_str());

        return Err(buf);   
    }
    
    // should be one OS (minimum) inside
    // unallocated partition must be larger than 20 GiB
    // fn _disk_check_requirements(&self) -> Result<bool, String> {
    //     let ctx = TeaPartitionGenerator::new(self.selected_blockdev.clone());
    //     let has_other_os = ctx.has_other_os(); // check 1

    //     let mut has_unallocated_space = false;

    //     let (start, end) = ctx.find_empty_space_sector_area();
    //     if start > 0 && end > 0 {
    //         has_unallocated_space = true;   // check 2
    //     }
        

    //     let sector_size = 0;
    //     if has_unallocated_space && has_other_os  {
    //         // check disk size
    //         let (start, end) = ctx.find_empty_space_sector_area();
    //         let size = end - start;

    //         let check_disk_layout = self.parted_partition_structure();
        
    //         if let Some(check_disk_layout_val) = check_disk_layout {
    //             let wanted_size: u64 = gb2sector(20, sector_size);

    //             if size >= wanted_size {
    //                 return Ok(true);
    //             } else {
    //                 return Err("You have other os & uninitialized free space, but its lower than 20 GB.".to_string());
    //             }
    //         } else {
    //             return Err("something error with parted_partition_structure.".to_string())
    //         }

            
    //     } else {
    //         return Err("your device didn't have secondary OS or free space".to_string())
    //     }

    // }

    fn get_highest_partition_number(&self, data: &Option<DiskInfo>) -> i32 {
        if let Some(data_val) = data {
            let mut highest: i32 = 1;
            // for x in data_val.disk.partitions.into_iter() {
            for x in <Vec<Partition> as Clone>::clone(&data_val.disk.partitions).into_iter() {
                if <u32 as TryInto<i32>>::try_into(x.number).unwrap() > highest {
                    highest = x.number.try_into().unwrap();
                }
            }
            return highest;
        }
        return -1;
    }

    fn _generate_json(&self, start: u64, end: u64) -> crate::blueprint::Storage {
        let ctx = TeaPartitionGenerator::new(self.selected_blockdev.clone());
        // let (start, end) = ctx.find_empty_space_sector_area(); // search for empty space
        let check_disk_layout = self.parted_partition_structure(); // found!, 

        let highest_disk = self.get_highest_partition_number(&check_disk_layout);

        // println!("{:#?}", check_disk_layout);
        println!("data {}", self.get_highest_partition_number(&check_disk_layout));
        // let partition_data = 

        let mut partition_data: Vec<crate::blueprint::Partition> = Vec::new();
        partition_data.push(
            crate::blueprint::Partition {
                number: (highest_disk + 1) as u64,       // next
                disk_path: Some(self.selected_blockdev.clone()),
                path: Some(format!("{}{}", self.selected_blockdev.clone(), highest_disk + 1)),
                mountpoint: Some("/".to_string()),
                filesystem: Some(self.selected_fs.clone()),
                format: true,
                start,
                end,
                size: end - start,
                label: None
            }
        );


        Storage {
            disk_path: Some(self.selected_blockdev.clone()),
            partition_table: Some(check_disk_layout.unwrap().disk.label.unwrap()),
            new_partition_table: false,
            layout_changed: false,
            autogenerated: true,
            autogenerated_mode: "doubleboot".to_string(),
            partitions: Some(partition_data),
            install_method: MethodKind::DUAL
        }
        // Storage::default()
    }


    fn getresult(&self, start: u64, end: u64) -> Result<crate::blueprint::Storage, String> {
        let check = self._check();

        match check {
            Ok(check_val) => {
                println!("{:#?}", check_val);
                // let check2 = self._disk_check_requirements();
                // match check2 {
                //     Ok(check2_val) => {
                        
                //     },
                //     Err(e) => {
                //         Err(e)
                //     }
                // }
                Ok(self._generate_json(start, end))
            }
            Err(e) => {
                let buf = format!("generation failed: {}", e);
                Err(buf)
            }
        }
        

    }
}
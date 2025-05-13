// this file is executed when user want Erase disk & clean install
use crate::blueprint::{Bootloader, Partition, Storage};
use crate::disk_helper::{gb2sector, mb2sector};
use crate::{config, exception, os};
use duct::cmd;
use serde::{Deserialize, Serialize};
use std::{clone, str::FromStr};
use tea_arch_chroot_lib::resource::MethodKind;

// karna di mode ini, user minta single boot & clean install hdd, maka kita butuh 2 struct
// karna seantero disk, partition table, dll semua diubah

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Partitions {
    node: String,
    start: u64,
    size: u64,
    #[serde(rename = "type")]
    _type: String,
    #[serde(default)]
    bootable: Option<bool>,

    // this is GPT spesific
    uuid: Option<String>,
    name: Option<String>,
    // end GPT spesific
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockDeviceData {
    pub label: String,
    pub id: String,
    pub device: String,
    pub unit: String,

    // this is GPT field
    pub firstlba: Option<u64>,
    pub lastlba: Option<u64>,
    // end gpt spesific field
    pub sectorsize: u64,
    pub partitions: Option<Vec<Partitions>>,
}

// this is sfdisk --json output
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartitionTable {
    pub partitiontable: BlockDeviceData,
}

#[derive(Debug, Clone)]
pub struct Blkstuff {
    pub selected_blockdev: String,
    pub selected_fs: String,
    pub selected_partition_table: String,
    pub partitiontable: PartitionTable,
    pub use_swap: bool,
}

pub trait SingleBootBlockdevice {
    fn blockdevice(blkname: String, fs: String, partition_table: String, use_swap: bool) -> Self;
    fn get_blkinfo(blkname: &String) -> Result<PartitionTable, String>;
    fn getblkbytes(&self) -> Option<u64>;
    fn getblksector(&self) -> Option<u64>;
    fn getresult(&self) -> Result<Storage, Box<dyn std::error::Error>>;
    fn gen_current_bootloader(&self) -> Option<Bootloader>;
    fn get_sector(&self, blkname: String) -> Result<u64, String>;
    fn _export_data(&self) -> ();
    // fn decide_swap_size(&self) -> u64;
}

impl SingleBootBlockdevice for Blkstuff {
    fn blockdevice(blkname: String, fs: String, partition_table: String, use_swap: bool) -> Self {
        let _blkdata: PartitionTable = Self::get_blkinfo(&blkname).unwrap_or_else(|e| {
            eprintln!("ERROR!!!!!!: {}", e);
            PartitionTable {
                partitiontable: BlockDeviceData {
                    label: "".to_string(),
                    id: "".to_string(),
                    device: "".to_string(),
                    unit: "".to_string(),
                    sectorsize: 0,
                    partitions: Some(Vec::new()),
                    firstlba: Some(0),
                    lastlba: Some(0),
                },
            }
        });

        Blkstuff {
            selected_blockdev: blkname,
            selected_fs: fs,
            selected_partition_table: partition_table,
            partitiontable: _blkdata,
            use_swap,
        }
    }

    fn get_sector(&self, blkname: String) -> Result<u64, String> {
        let sfdisk_res = cmd!("blockdev", "--getss", blkname).read();
        if let Ok(sfdisk_res_val) = sfdisk_res {
            let conv = sfdisk_res_val.parse::<u64>();

            if let Ok(conv_val) = conv {
                Ok(conv_val)
            } else {
                Err("blockdev getss fail str to int conversion".to_string())
            }
        } else {
            Err("blockdev getss failed".to_string())
        }
    }

    fn get_blkinfo(blkname: &String) -> Result<PartitionTable, String> {
        let sfdisk_res = cmd!("sfdisk", "--json", blkname).read();

        if let Ok(sfdisk_res_val) = sfdisk_res {
            let sfdisk_parsed: Result<PartitionTable, serde_json::Error> =
                serde_json::from_str::<PartitionTable>(&sfdisk_res_val);
            // self.blkdata = sfdisk_parsed;

            match sfdisk_parsed {
                Ok(val) => Ok(val),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("sfdisk error!!!!".to_string())
        }
    }

    /// this func return how many bytes of data
    fn getblkbytes(&self) -> Option<u64> {
        let data = cmd!("blockdev", "--getsize64", self.selected_blockdev.clone()).read();
        // println!("{:#?}", data);

        if let Ok(data_val) = data {
            let ret = u64::from_str(&data_val).unwrap();
            // println!("convert {:#?}", dat);

            Some(ret)
        } else {
            None
        }
    }

    /// this func return how many sector of disk
    fn getblksector(&self) -> Option<u64> {
        let data = cmd!("blockdev", "--getsz", self.selected_blockdev.clone()).read();
        // println!("{:#?}", data);

        if let Ok(data_val) = data {
            let ret = u64::from_str(&data_val).unwrap();
            // println!("convert {:#?}", dat);

            Some(ret)
        } else {
            None
        }
    }

    

    fn getresult(&self) -> Result<Storage, Box<dyn std::error::Error>> {
        // let Ok(blksize) = self.partitiontable.partitiontable.sectorsize;
        let current_size = self.getblkbytes();
        let current_size_sector = self.getblksector();
        let current_sector = self.get_sector(self.selected_blockdev.clone())?;

        // this func itended to return as json
        let mut disks_export: Vec<Partition> = Vec::new();

        let _current_size_val = match current_size {
            Some(size) => size,
            None => {
                return Err(Box::new(exception::TealinuxAutoPartitionErr::InternalErr(
                    "something error with getblkbytes()".to_string(),
                )));
            }
        };

        // larger than 20 gb
        if current_size.unwrap() > (20 * 1024 * 1024 * 1024) {
            // setup 512 MB for GPT stuff
            // let mut last_sector: u64 = gb2sector(70, self.partitiontable.partitiontable.sectorsize);

            let mut counter = 1;
            if self.selected_partition_table.to_lowercase() == "gpt" {

                disks_export.push(Partition {
                    number: counter,
                    disk_path: Some(self.selected_blockdev.clone()),
                    path: Some(format!("{}{}", self.selected_blockdev.clone(), counter)),
                    mountpoint: Some("/boot/efi".to_string()),
                    filesystem: Some("fat32".to_string()),
                    label: None,
                    format: true,
                    start: 2048, // aligment
                    end: 2048 + mb2sector(512, current_sector),
                    size: mb2sector(512, current_sector),
                });
                counter = counter + 1;

                // align + size (prev)
                let mut last_sector: u64 = 2048 + mb2sector(512, current_sector);

                if self.use_swap {
                    let swap_size = os::Os::decide_swap_size2(self.selected_blockdev.clone()).unwrap();
                    
                    disks_export.push(Partition {
                        number: counter,
                        disk_path: Some(self.selected_blockdev.clone()),
                        path: Some(format!("{}{}", self.selected_blockdev.clone(), counter)),
                        mountpoint: None,
                        filesystem: Some("linux-swap".to_string()),
                        format: true,
                        start: last_sector + 1,
                        end: last_sector + mb2sector(swap_size, current_sector),
                        size: mb2sector(swap_size, current_sector),
                        label: None,
                    });

                    last_sector = last_sector + mb2sector(swap_size, current_sector);
                    counter = counter + 1;
                }

                // this is root partition
                disks_export.push(Partition {
                    number: counter,
                    disk_path: Some(self.selected_blockdev.clone()),
                    path: Some(format!("{}{}", self.selected_blockdev.clone(), counter)),
                    mountpoint: Some("/".to_string()), // some exception if BTRFS is used, this is unneed
                    filesystem: Some(self.selected_fs.to_string()),
                    label: None,
                    format: true,
                    start: last_sector + 1,
                    end: current_size_sector.unwrap() - 2048,
                    size: current_size_sector.unwrap() - last_sector - 2048,
                });
            } else {
                let mut last_sector: u64 = 2048;

                if self.use_swap {
                    let swap_size = os::Os::decide_swap_size2(self.selected_blockdev.clone()).unwrap();
                    
                    disks_export.push(Partition {
                        number: counter,
                        disk_path: Some(self.selected_blockdev.clone()),
                        path: Some(format!("{}{}", self.selected_blockdev.clone(), counter)),
                        mountpoint: None,
                        filesystem: Some("linux-swap".to_string()),
                        format: true,
                        start: last_sector + 1,
                        end: last_sector + mb2sector(swap_size, current_sector),
                        size: mb2sector(swap_size, current_sector),
                        label: None,
                    });

                    last_sector = last_sector + mb2sector(swap_size, current_sector);
                    counter = counter + 1;
                }

                disks_export.push(Partition {
                    number: counter,
                    disk_path: Some(self.selected_blockdev.clone()),
                    path: Some(format!("{}1", self.selected_blockdev.clone())),
                    mountpoint: Some("/".to_string()),
                    filesystem: Some(self.selected_fs.to_string()),
                    label: None,
                    format: true,
                    start: last_sector + 1, // aligment
                    end: current_size_sector.unwrap() - 2048,
                    size: current_size_sector.unwrap() - (last_sector + 1),
                });
            }

            // disk lower than 20 GB
        } else {
            return Err(Box::new(
                exception::TealinuxAutoPartitionErr::InsufficientStorage(
                    "Selected storage is lower than 20 GB, Aborted!!".to_string(),
                ),
            ));
        }

        // println!("DEBUG current_selected_block_size: {:#?}", current_size);
        // println!("{:#?}", disks_export);

        Ok(Storage {
            disk_path: Some(self.selected_blockdev.clone()),
            partition_table: Some(self.selected_partition_table.clone()),
            new_partition_table: true,
            layout_changed: true,
            autogenerated: true,
            autogenerated_mode: "singleboot".to_string(),
            partitions: Some(disks_export),
            install_method: MethodKind::SINGLE,
        })
        // return Err(Box::new(
        //     error::TealinuxAutoPartitionErr::InsufficientStorage(
        //         "something error with getblkbytes()".to_string(),
        //     ),
        // ));

        // if current_size_val < (self.partitiontable.partitiontable.sectorsize * 1024 * 1024) {
        // Err(Box::new(
        //     error::TealinuxAutoPartitionErr::InsufficientStorage(
        //         "check your storages size".to_string(),
        //     ),
        // ))
        // } else {
        //     // ONLY if disk larger than 256 GB
        //     Err(Box::new(
        //         error::TealinuxAutoPartitionErr::InsufficientStorage(
        //             "check your storage size".to_string(),
        //         ),
        //     ))
        // }
    }

    fn _export_data(&self) -> () {
        println!("{:#?}", self.partitiontable);
    }

    fn gen_current_bootloader(&self) -> Option<Bootloader> {
        if self.selected_partition_table.to_lowercase() == "mbr" {
            Some(Bootloader {
                firmware_type: tea_arch_chroot_lib::resource::FirmwareKind::BIOS,
                path: Some(self.selected_blockdev.clone()),
            })
        } else {
            Some(Bootloader {
                firmware_type: tea_arch_chroot_lib::resource::FirmwareKind::UEFI,
                path: Some(format!("{}1", self.selected_blockdev.clone())),
            })
        }
    }

    // fn convert_block2bytes()
}

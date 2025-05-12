// ref: https://github.com/tealinuxos/tea-arch-chroot-lib/blob/master/src/chroot/os.rs
// by: Gagah Syuja

use crate::disk_helper;
use duct::cmd;
use lazy_regex::regex_captures;
use serde;
use serde::Serialize;
use std::error;
use std::str::FromStr;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

#[derive(Serialize, std::fmt::Debug)]
#[serde(rename_all = "camelCase")]
pub struct Os {
    pub name: String,
    pub path: String,
}

impl Os {
    pub fn get_other_os() -> Result<Option<Vec<Self>>, Box<dyn error::Error>> {
        let mut oses: Vec<Self> = Vec::new();

        // let prober = cmd!("os-prober").read()?;

        // For testing purposes
        let prober = concat!(
            "/dev/sdd1@/efi/Microsoft/Boot/bootmgfw.efi:Windows Boot Manager:Windows:efi\n",
            "/dev/sdb2@/efi/Microsoft/Boot/bootmgfw.efi:Wondows Boot Manager:Windows:efi\n",
            "/dev/nvme0n1p1@/efi/Microsoft/Boot/bootmgfw.efi:Windows Boot Manager:Windows:efi"
        );

        let entries: Vec<String> = prober.split("\n").map(|s| s.to_string()).collect();

        for entry in entries {
            let result = regex_captures!(r"(\/dev\/[^\@]+)\@[^:]*:([^:]+)", &entry);

            if let Some(result) = result {
                let path = result.1;
                let name = result.2;

                oses.push(Os {
                    name: name.to_string(),
                    path: path.to_string(),
                });
            }
        }

        if oses.is_empty() {
            Ok(None)
        } else {
            Ok(Some(oses))
        }
    }

    pub fn get_total_memory() -> u64 {
        let sysinfo = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
        );

        sysinfo.total_memory() / 1024000
    }

    fn __gen_mem_ideal_size(memory: u64) -> u64 {
        let ideal_size = match memory {
            m if m < 8192 => m * 2,
            m if m < 16384 => ((m as f64 * 1.5) as usize).try_into().unwrap(),
            m if m < 32768 => m,
            m if m >= 32768 => m / 2,
            _ => memory,
        };

        ideal_size
    }

    pub fn decide_swap_size() -> u64 {
        let memory = crate::os::Os::get_total_memory();

        let ideal_size = Self::__gen_mem_ideal_size(memory);

        ideal_size
    }

    pub fn decide_swap_size2(device: String) -> Option<u64> {
        let memory = crate::os::Os::get_total_memory();

        let ideal_size = Self::__gen_mem_ideal_size(memory);

        let data = cmd!("blockdev", "--getsize64", device).read();
        // println!("{:#?}", data);

        if let Ok(data_val) = data {
            let ret = u64::from_str(&data_val).unwrap();
            let ret_mb = disk_helper::bytes2mb(ret) as f64;
            let mem_upper_limit = (ret_mb as f64) * (32.0 / 100.0);

            if ideal_size as f64 > mem_upper_limit {
                // println!("max swap : {}", mem_upper_limit);
                Some(mem_upper_limit as u64)
            } else {
                // println!("max swap : {}", ideal_size);
                Some(ideal_size as u64)
            }
        } else {
            None
        }
    }

    /// this func return mb
    pub fn decide_swap_size2_bytes(bytes_size: u64) -> Option<u64> {
        let memory = crate::os::Os::get_total_memory();
        let ideal_size = Self::__gen_mem_ideal_size(memory);

        let ret_mb = disk_helper::bytes2mb(bytes_size) as f64;
        let mem_upper_limit = (ret_mb as f64) * (32.0 / 100.0);

        if ideal_size as f64 > mem_upper_limit {
            // println!("max swap : {}", mem_upper_limit);
            Some(mem_upper_limit as u64)
        } else {
            // println!("max swap : {}", ideal_size);
            Some(ideal_size as u64)
        }
    }

    pub fn get_sector(blkname: String) -> Result<u64, String> {
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
}

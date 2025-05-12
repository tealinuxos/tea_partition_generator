// ref: https://github.com/tealinuxos/tea-arch-chroot-lib/blob/master/src/chroot/os.rs
// by: Gagah Syuja

use std::error;
use lazy_regex::regex_captures;
use serde::Serialize;
use duct::cmd;
use serde;
use sysinfo::{ System, RefreshKind, MemoryRefreshKind };
use std::str::FromStr;
use crate::disk_helper;

#[derive(Serialize, std::fmt::Debug)]
#[serde(rename_all="camelCase")]
pub struct Os
{
    pub name: String,
    pub path: String
}

impl Os
{
    pub fn get_other_os() -> Result<Option<Vec<Self>>, Box<dyn error::Error>>
    {
        let mut oses: Vec<Self> = Vec::new();

        // let prober = cmd!("os-prober").read()?;

        // For testing purposes
        let prober = concat!(
            "/dev/sdd1@/efi/Microsoft/Boot/bootmgfw.efi:Windows Boot Manager:Windows:efi\n",
            "/dev/sdb2@/efi/Microsoft/Boot/bootmgfw.efi:Wondows Boot Manager:Windows:efi\n",
            "/dev/nvme0n1p1@/efi/Microsoft/Boot/bootmgfw.efi:Windows Boot Manager:Windows:efi"
        );

        let entries: Vec<String> = prober
            .split("\n")
            .map(|s| s.to_string())
            .collect();

        for entry in entries
        {
            let result = regex_captures!(r"(\/dev\/[^\@]+)\@[^:]*:([^:]+)", &entry);

            if let Some(result) = result
            {
                let path = result.1;
                let name = result.2;

                oses.push(Os {
                    name: name.to_string(),
                    path: path.to_string()
                });
            }
        }

        if oses.is_empty()
        {
            Ok(None)
        }
        else
        {
            Ok(Some(oses))
        }
    }

    pub fn get_total_memory() -> u64 {
        let sysinfo = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram())
        );

        sysinfo.total_memory() / 1024000
    }

    pub fn decide_swap_size() -> u64 {
        let memory = crate::os::Os::get_total_memory();

        let ideal_size = match memory {
            m if m < 8192 => m * 2,
            m if m < 16384 => ((m as f64 * 1.5) as usize).try_into().unwrap(),
            m if m < 32768 => m,
            m if m >= 32768 => m / 2,
            _ => memory,
        };

        ideal_size
        
    }

    pub fn decide_swap_size2(device: String) -> Option<u64> {
        let memory = crate::os::Os::get_total_memory();
    
        let ideal_size = match memory {
            m if m < 8192 => m * 2,
            m if m < 16384 => ((m as f64 * 1.5) as usize).try_into().unwrap(),
            m if m < 32768 => m,
            m if m >= 32768 => m / 2,
            _ => memory,
        };
    
        let data = cmd!("blockdev", "--getsize64", device).read();
        // println!("{:#?}", data);
    
        if let Ok(data_val) = data {
            let ret = u64::from_str(&data_val).unwrap();
            let ret_mb = disk_helper::bytes2mb(ret) as f64;
            let mem_upper_limit = (ret_mb as f64)  * (32.0 / 100.0);
    
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
}
// ref: https://github.com/tealinuxos/tea-arch-chroot-lib/blob/master/src/chroot/os.rs
// by: Gagah Syuja

use std::fs::{read_to_string, write};
use std::io::{self, Write};
use std::path::Path;

use crate::blueprint::Storage;
use crate::disk_helper;
use duct::cmd;
use lazy_regex::regex_captures;
use serde;
use serde::Serialize;
use std::error;
use std::str::FromStr;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use tea_arch_chroot_lib::resource::MethodKind;

use std::fs;
use std::fs::File;

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
            "/dev/nvme0n1p1@/efi/Microsoft/Boot/bootmgfw.efi:Windows Boot Manager:Windows:efi\n",
            "/dev/sda1:Windows 10:Winlost:chain"
        );

        let entries: Vec<String> = prober.split("\n").map(|s| s.to_string()).collect();

        for entry in entries {
            // let result = regex_captures!(r"(\/dev\/[^\@]+)\@[^:]*:([^:]+)", &entry);
            let result = regex_captures!(r"^(\/dev\/[^\s:@]+)(?:@[^:]+)?:([^:]+):", &entry);

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

    pub fn get_disk_model(data: String) -> Result<String, String> {
        let lsblkout = cmd!("lsblk", data, "-J", "-o", "NAME,MODEL").read();

        if let Ok(lsblkout_val) = lsblkout {
            let v: serde_json::Value = serde_json::from_str(&lsblkout_val).unwrap();

            return Ok(v["blockdevices"][0]["model"].as_str().unwrap().to_string());
        }

        Err("get_disk_model: call lsblk fail".to_string())
    }

    fn __append_swap_fstab(data: &Storage) -> Option<String> {
        if let Some(partitions_val) = &data.partitions {
            for partition_i in partitions_val {
                if partition_i.filesystem == Some("linux-swap".to_string()) {
                    let fstab_str =
                        format!("{} none swap sw 0 0", partition_i.path.clone().unwrap());

                    return Some(fstab_str);
                }
            }
            return None;
        } else {
            return None;
        }
    }

    pub fn append_swap_fstab(data: &Storage) -> Result<(), String> {
        let fstab_ret = Self::__append_swap_fstab(data);

        if let Some(fstab_val) = fstab_ret {
            println!("appending: {}", fstab_val.clone());

            let mut fd = File::options()
                .append(true)
                .open("/tealinux-mount/etc/fstab");

            if let Ok(mut fd_val) = fd {
                writeln!(&mut fd_val, "{}", fstab_val.clone().as_str());
            } else {
                return Err(
                    "something wrong with file descriptor during appending swap fstab!".to_string(),
                );
            }
        } else {
            return Err("appending fstab swap failed".to_string());
        }

        Ok(())
    }

    pub fn patch_grub_config_disable_os_probe(val_nostr: bool) {
        let key = "GRUB_DISABLE_OS_PROBER";
        let val = format!("{}", val_nostr);

        let path_str = "/tealinux-mount/etc/default/grub";

        let content = read_to_string(path_str);

        if let Ok(content_val) = content {
            //
            let mut lines: Vec<String> = content_val.lines().map(|l| l.to_string()).collect();
            let mut found = false;

            for line in lines.iter_mut() {
                if line.trim_start().starts_with(&format!("{key}="))
                    || line.trim_start().starts_with(&format!("#{key}="))
                {
                    *line = format!("{key}={val}");
                    found = true;
                    break;
                }
            }

            if !found {
                lines.push(format!("{key}={val}"));
            }

            // Join lines and write back
            let new_content = lines.join("\n") + "\n";
            let _ = write(path_str, new_content);
        } else {
            println!("read {} failed, aborted", path_str);
        }
    }

    pub fn regenerate_grub() {
        let _ = cmd!("grub-mkconfig", "-o", "/boot/grub/grub.cfg").run();
    }

    pub fn align_2048(value: u64) -> u64 {
        let alignment: u64 = 2048;
        (value + alignment - 1) & !(alignment - 1)
    }
}

#[derive(Debug, Clone)]
struct _InternalDiskNum {
    partition: u32,
    mark: bool,
}

#[derive(Debug, Clone)]
pub struct StateDiskPredictor {
    disk: String,
    firmware_mode: String,
    slot: Vec<_InternalDiskNum>,
}

pub trait DiskPredictor {
    fn new(disk: String, mode: String) -> Self;
    fn predict_next_disk(&mut self) -> Option<u32>;
    fn mark(&mut self, disk_num: u32);

    fn get_disk_num_array(device: String) -> Result<Vec<u32>, String>;
    fn predict_next_disks_num(&mut self) -> Option<u32>;

    fn _debug(&mut self);
}

impl DiskPredictor for StateDiskPredictor {
    fn new(disk: String, mode: String) -> Self {
        let mut buf: Vec<_InternalDiskNum> = Vec::new();

        if mode.to_lowercase() == "mbr" {
            // buf = (1..=4).collect();
            buf = (1..=4)
                .map(|n| _InternalDiskNum {
                    partition: n,
                    mark: false,
                })
                .collect()
        } else {
            // buf = (1..=128).collect();
            buf = (1..=128)
                .map(|n| _InternalDiskNum {
                    partition: n,
                    mark: false,
                })
                .collect()
        }

        StateDiskPredictor {
            disk: disk,
            firmware_mode: mode,
            slot: buf,
        }
    }

    fn predict_next_disk(&mut self) -> Option<u32> {
        self.predict_next_disks_num()
    }

    fn mark(&mut self, disk_num: u32) {
        for x in &mut self.slot {
            if x.partition == disk_num {
                x.mark = true;
            }
        }
    }

    fn get_disk_num_array(device: String) -> Result<Vec<u32>, String> {
        let data = cmd!("parted", device, "-j", "--script", "print").read();

        if let Ok(data_val) = data {
            let parted_json: serde_json::Result<serde_json::Value> =
                serde_json::from_str(&data_val);

            if let Ok(parted_json_val) = parted_json {
                // let buf: Vec<u32> = Vec::new();

                // for x in parted_json_val["disk"]["partitions"] {
                //     buf.push(x["number"])
                // }

                let numbers: Vec<u32> = parted_json_val["disk"]["partitions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|p| p["number"].as_u64().map(|n| n as u32))
                    .collect();

                // println!("{:?}", numbers);
                // return Err("ok".to_string());
                Ok(numbers)
            } else {
                return Err("get_disk_num_array parsing json failed".to_string());
            }
        } else {
            return Err("get_disk_num_array call parted failed".to_string());
        }
    }

    fn predict_next_disks_num(&mut self) -> Option<u32> {
        let partnum = Self::get_disk_num_array(self.disk.clone());

        if let Ok(partnum_val) = partnum {
            for x in &self.slot {
                if !partnum_val.contains(&x.partition) && x.mark == false {
                    return Some(x.partition);
                }
            }
        }
        return None;
    }

    fn _debug(&mut self) {
        println!("{:?}", self);
        println!("{:?}", Self::get_disk_num_array(self.disk.clone()));
    }
}




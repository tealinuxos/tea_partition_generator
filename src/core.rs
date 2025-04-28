use crate::os;
use async_trait::async_trait;
use serde_json::{Error, Result};
use crate::disk_helper;
use crate::parted_parser;
use duct::cmd;
pub struct TeaPartitionGenerator {
    selected: String,
}

// this struct bring such
// /dev/sdb instead partition like /dev/sdb3 
// whatever it is

#[derive(Debug)]
pub struct OsOnDisk
{
    pub name: String,
    pub path: String
}

impl TeaPartitionGenerator {
    pub fn new(selected: String) -> TeaPartitionGenerator {
        TeaPartitionGenerator { selected }
    }

    fn _os_probe_output_find_other_os(&self, data: &Vec<os::Os>) -> bool {
        let mut found = false;
        for data_i in data {
            if data_i.path.starts_with("/dev/nvme") {
                // handle nvme
                let ret = disk_helper::nvme_split(data_i.path.clone());
                if let Some(ret_val) = ret {
                    if ret_val.0 == self.selected {
                        found = true;
                    }
                }
            } else if data_i.path.starts_with("/dev/sd") { // scsi 

                let ret = disk_helper::scsi_split(data_i.path.clone());
                if let Some(ret_val) = ret {
                    if ret_val == self.selected {
                        found = true;
                    }
                }
            }
        }
        found
    }
}

#[async_trait]
pub trait PartitionGenerator {
    async fn has_other_os(&self) -> bool;
    async fn disk_list_other_os() -> Option<Vec<OsOnDisk>>;
    async fn find_empty_space_sector_area(&self) -> Option<(u64, u64)>;
}

#[async_trait]
impl PartitionGenerator for TeaPartitionGenerator {
    async fn has_other_os(&self) -> bool {
        let ret = os::Os::get_other_os().await;

        if let Ok(ret_val) = ret {
            if let Some(data) = ret_val {

                // check if osprobe output contain selected disks
                // for data_i in &data {
                //     println!("{:?}", data_i.path );
                //     // if 
                // }
                // _parse_os_probe_match(&data);
                let ret = self._os_probe_output_find_other_os(&data);
                // println!("fn {:?}", ret);
                ret
            } else {
                false
            }
        } else {
            false
        }
    }

    // convention: start ~ end
    async fn find_empty_space_sector_area(&self) -> Option<(u64, u64)> {
        // the disk must be larger than 7 GiB (currently)
        // let run = format!(, self.selected);
        let parted = cmd!("sudo", "parted", "-m", self.selected.clone(), "unit", "s", "print", "free").read();

        if let Ok(parted_data) = parted {
            let ret = parted_parser::PartedResult::parse(parted_data);

            for parted_data_i in &ret.data {
                // NOTE: Tunning this number
                if ((ret.info.sector_size_logical as u64) * parted_data_i.size) > 7516192768 && parted_data_i.fs == "free" {
                    return Some((parted_data_i.start, parted_data_i.end))
                }
            }

            // println!("{:#?}", ret);
        }

        Some((0,0))
    }

    // this func return 
    // 
    // example:
    // /dev/sdb instead /dev/sdb3 (in os prober output)
    // and make sure if the os prober output is match with current partition layout
    async fn disk_list_other_os() -> Option<Vec<OsOnDisk>> {
        let ret = os::Os::get_other_os().await;

        let mut buf: Vec<OsOnDisk> = Vec::new();

        if let Ok(ret_val) = ret {
            if let Some(data) = ret_val {
                for data_i in data {
                    if data_i.path.starts_with("/dev/nvme") {
                        // handle nvme
                        let ret = disk_helper::nvme_split(data_i.path.clone());
                        if let Some(ret_val) = ret {
                            buf.push(OsOnDisk {
                                path: ret_val.0,
                                name: data_i.name
                            });

                        }
                    } else if data_i.path.starts_with("/dev/sd") { // scsi 
        
                        let ret = disk_helper::scsi_split(data_i.path.clone());
                        if let Some(ret_val) = ret {
                            buf.push(OsOnDisk {
                                path: ret_val,
                                name: data_i.name
                            });
                        }
                    }
                }
            }
        }

        if buf.len() == 0 {
            None
        } else {
            Some(buf)
        }
    }
     
}

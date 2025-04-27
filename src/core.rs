use crate::os;
use async_trait::async_trait;
use serde_json::{Error, Result};
use crate::disk_helper;
pub struct TeaPartitionGenerator {
    selected: String,
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
}

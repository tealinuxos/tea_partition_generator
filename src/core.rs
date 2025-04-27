use crate::os;
use async_trait::async_trait;
use serde_json::{Error, Result};

pub struct TeaPartitionGenerator {
    selected: String,
}

impl TeaPartitionGenerator {
    pub fn new(selected: String) -> TeaPartitionGenerator {
        TeaPartitionGenerator { selected }
    }
}

#[async_trait]
pub trait PartitionGenerator {
    async fn has_other_os(&self) -> bool;
}

#[async_trait]
impl PartitionGenerator for TeaPartitionGenerator {
    // async fn _parse_os_probe_match(data: Vec<Self>) -> bool {

    // }

    async fn has_other_os(&self) -> bool {
        let ret = os::Os::get_other_os().await;

        if let Ok(ret_val) = ret {
            if let Some(data) = ret_val {

                println!("{:?}", data);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

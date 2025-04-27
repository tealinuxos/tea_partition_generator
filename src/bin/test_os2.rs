use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

#[tokio::main]
async fn main() {
    let ret = TeaPartitionGenerator::new("/dev/nvme0n1".to_string());
    ret.has_other_os().await;
}
use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

#[tokio::main]
async fn main() {
    // let ctx = TeaPartitionGenerator::new("/dev/sda".to_string());

    let has_other_os = TeaPartitionGenerator::disk_list_other_os().await;
    println!("Has other os: {:#?}", has_other_os);
}
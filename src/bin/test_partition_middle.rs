use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

#[tokio::main]
async fn main() {
    let ctx = TeaPartitionGenerator::new("/dev/sdb".to_string());

    let has_other_os = ctx.find_empty_space_sector_area().await;
    println!("Has other os: {:?}", has_other_os);
}
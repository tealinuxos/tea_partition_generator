use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

fn main() {
    let ctx = TeaPartitionGenerator::new("/dev/sdb".to_string());
    ctx.find_partition_sector_areav();
}
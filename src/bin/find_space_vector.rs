use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

fn main() {
    let ctx = TeaPartitionGenerator::new("/dev/sdb".to_string());


    let data = ctx.find_empty_space_sector_areav();
    println!("{:#?}", data);
}
use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

fn main() {
    let ctx = TeaPartitionGenerator::new("/dev/sdb".to_string());

    let has_other_os = ctx.has_other_os();
    println!("Has other os: {}", has_other_os);
    let (start, end) = ctx.find_empty_space_sector_area();

    // if let Some((start, end) = data {

        println!("empty SCSI space at block {} ~ {}", start, end);
    // }
}
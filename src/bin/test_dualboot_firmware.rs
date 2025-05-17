use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};
use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

fn main() {
    let mut ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "btrfs".to_string(),
        true
    );

    let partition_generator_ctx = TeaPartitionGenerator::new("/dev/sdb".to_string());
    let (start, end) = partition_generator_ctx.find_empty_space_sector_area();

    // println!("partition design: {} {}", start, end);
    // return;
    if start == 0 && end == 0 {
        println!("no empty partition, aborting!");
        return;
    }

    ctx.getresult(start, end);
    let ret = ctx.gen_current_bootloader();

    println!("{:?}", ret);
}
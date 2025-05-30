use tea_arch_chroot_lib::resource::MethodKind;
use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};
use tea_partition_generator::mkpart::Partgen;
use tea_partition_generator::os::Os;
use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

#[tokio::main]
async fn main() {
    let mut ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(),
        true
    );

    // this number come from FE, use find_empty_space_sector_areav
    let partition_generator_ctx = TeaPartitionGenerator::new("/dev/sdb".to_string());
    let (start, end) = partition_generator_ctx.find_empty_space_sector_area();

    // println!("partition design: {} {}", start, end);
    // return;
    if start == 0 && end == 0 {
        println!("no empty partition, aborting!");
        return;
    }

    let mut ret = ctx.getresult(start, end);

    println!("runn {:#?}", ret);


}
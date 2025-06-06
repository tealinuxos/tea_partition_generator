use tea_arch_chroot_lib::resource::MethodKind;
use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};
use tea_partition_generator::mkpart::Partgen;
use tea_partition_generator::os::Os;
use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

#[tokio::main]
async fn main() {
    let mut ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/nvme0n1".to_string(), 
        "ext4".to_string(),
        true
    );

    let start = 81203200;
    let end = 167772159;


    if start == 0 && end == 0 {
        println!("no empty partition, aborting!");
        return;
    }

    let ret = ctx.getresult(start, end);
    println!("{:#?}", ret);

}
use tea_partition_generator::single_boot_blockdev::{
    SingleBootBlockdevice,
    Blkstuff
};

use tea_partition_generator::blueprint::Partition;

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/nvme0n1".to_string(), 
        "ext4".to_string(), 
        "mbr".to_string(),
        true
    );

    let ret = ctx.getresult();
    println!("{:#?}", ret);

}
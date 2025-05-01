use tea_partition_generator::single_boot_blockdev::{
    SingleBootBlockdevice,
    Blkstuff
};

use tea_partition_generator::blueprint::Partition;

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "btrfs".to_string(), 
        "mbr".to_string()
    );

    let ret = ctx.gen_current_bootloader();
    println!("{:#?}", ret);

}
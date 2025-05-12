use tea_partition_generator::single_boot_blockdev::{
    SingleBootBlockdevice,
    Blkstuff
};

use tea_partition_generator::blueprint::Partition;

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "btrfs".to_string(), 
        "gpt".to_string(),
        true
    );

    let ret = ctx.getresult();
    println!("{:#?}", ret);

}
use tea_partition_generator::single_boot_blockdev::{
    SingleBootBlockdevice,
    Blkstuff
};

use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::mkpart::Partgen;

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(), 
        "mbr".to_string(),
        false
    );

    let ret = ctx.getresult();
    // println!("{:#?}", ret);

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(
            ret_val.clone(), ret_val.install_method.clone()
        );
    }

}
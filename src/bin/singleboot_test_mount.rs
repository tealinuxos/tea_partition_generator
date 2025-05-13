use tea_partition_generator::single_boot_blockdev::{Blkstuff, SingleBootBlockdevice};

use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::mkpart::Partgen;
use tea_partition_generator::mounting::{Mount, MountPoint};

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(),
        "ext4".to_string(),
        "mbr".to_string(),
        true,
    );

    let ret = ctx.getresult();
    // println!("{:#?}", ret);

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(ret_val.clone(), ret_val.install_method.clone());

        let mnt = MountPoint::new(ret_val);
        mnt.mount_all();
        // println!("{:#?}", mnt);
    }
}

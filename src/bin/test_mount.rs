use tea_partition_generator::single_boot_blockdev::{
    SingleBootBlockdevice,
    Blkstuff
};

use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::mounting::{Mount, MountPoint};

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(), 
        "gpt".to_string()
    );

    let ret = ctx.getresult();

    if let Ok(data) = ret {
        let mnt = MountPoint::new(data);
        mnt.mount_all();
        println!("{:#?}", mnt);
    } else {
        println!("err: {:#?}", ret);
    }

}

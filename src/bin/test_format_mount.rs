use tea_partition_generator::single_boot_blockdev::{
    SingleBootBlockdevice,
    Blkstuff
};

use tea_partition_generator::mounting::{Mount, MountPoint};
use tea_partition_generator::mkpart::Partgen;

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(), 
        "gpt".to_string()
    );

    let ret = ctx.getresult();

    if let Ok(data) = ret {
        Partgen::do_dangerous_task_on(
            data.clone()
        );

        let mnt = MountPoint::new(data.clone());
        mnt.mount_all();
        println!("{:#?}", mnt);
    } else {
        println!("err: {:#?}", ret);
    }

}

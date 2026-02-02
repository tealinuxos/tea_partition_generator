use tea_partition_generator::single_boot_blockdev::{
    SingleBootBlockdevice,
    Blkstuff
};

use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::mounting::{Mount, MountPoint};
use tea_partition_generator::mkpart::Partgen;
use tea_arch_chroot_lib::resource::MethodKind;


fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(), 
        "mbr".to_string(),
        false
    );

    let ret = ctx.getresult();

    // println!("{:#?}", ret);

    if let Ok(data) = ret {
        Partgen::do_dangerous_task_on(
            data.clone(), MethodKind::SINGLE
        );

        let mnt = MountPoint::new(data.clone());
        mnt.mount_all();
        println!("{:#?}", mnt);
    } else {
        println!("err: {:#?}", ret);
    }

}

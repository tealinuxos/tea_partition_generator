use tea_arch_chroot_lib::chroot::os;
use tea_partition_generator::single_boot_blockdev::{Blkstuff, SingleBootBlockdevice};

use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::mkpart::Partgen;
use tea_partition_generator::mounting::{Mount, MountPoint};
use tea_partition_generator::os::Os;

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(),
        "ext4".to_string(),
        "mbr".to_string(),
        true,
    );

    let ret = ctx.getresult();
    // println!("{:#?}", ret);

    if let Ok(ret_val) = &ret {
        let fstab = Os::append_swap_fstab(ret_val);
    }


}

use tea_partition_generator::single_boot_blockdev::{
    SingleBootBlockdevice,
    Blkstuff
};

use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::mkpart::Partgen;

fn test_ext4_gpt() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(), 
        "gpt".to_string()
    );

    let ret = ctx.getresult();

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(
            ret_val
        );
    }
}

fn test_btrfs_gpt() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "btrfs".to_string(), 
        "gpt".to_string()
    );

    let ret = ctx.getresult();

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(
            ret_val
        );
    }
}

fn test_ext4_mbr() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(), 
        "mbr".to_string()
    );

    let ret = ctx.getresult();

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(
            ret_val
        );
    }
}

fn test_btrfs_mbr() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "btrfs".to_string(), 
        "mbr".to_string()
    );

    let ret = ctx.getresult();

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(
            ret_val
        );
    }
}

fn main() {
    // self::test_ext4_gpt();
    // self::test_btrfs_gpt();
    // self::test_ext4_mbr();
    self::test_btrfs_mbr();
}
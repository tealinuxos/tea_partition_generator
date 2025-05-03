use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};

#[tokio::main]
async fn main() {
    let ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string() 
    );

    println!("Has other os: {:#?}", ctx.check_base_disk_layout());
}
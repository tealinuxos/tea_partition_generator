use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};

#[tokio::main]
async fn main() {
    let ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sda".to_string(), 
        "ext4".to_string() 
    );

    println!("partition design: {:#?}", ctx.getresult());
}
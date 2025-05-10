use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};

#[tokio::main]
async fn main() {
    let ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string() 
    );

    // this number come from FE, use find_empty_space_sector_areav
    println!("partition design: {:#?}", ctx.getresult(3624960, 20813823));
}
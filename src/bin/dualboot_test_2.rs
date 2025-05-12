use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};

#[tokio::main]
async fn main() {
    let ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(),
        true
    );

    // this number come from FE, use find_empty_space_sector_areav
    println!("partition design: {:#?}", ctx.getresult(13088768, 50331647));
}
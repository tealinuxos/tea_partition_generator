use tea_arch_chroot_lib::resource::MethodKind;
use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};
use tea_partition_generator::mkpart::Partgen;

#[tokio::main]
async fn main() {
    let ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(),
        true
    );

    // this number come from FE, use find_empty_space_sector_areav
    // println!("partition design: {:#?}", );
    let ret = ctx.getresult(13088768, 50331647);

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(
            ret_val, MethodKind::DUAL
        );
    }
}
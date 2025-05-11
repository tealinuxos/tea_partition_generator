use tea_arch_chroot_lib::resource::MethodKind;
use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};
use tea_partition_generator::mkpart::Partgen;

#[tokio::main]
async fn main() {
    let ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "btrfs".to_string()
    );

    // this number come from FE, use find_empty_space_sector_areav
    // println!("partition design: {:#?}", );
    let ret = ctx.getresult(3624960, 20813823);

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(
            ret_val, MethodKind::DUAL
        );
    }
}
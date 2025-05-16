use tea_arch_chroot_lib::resource::MethodKind;
use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};
use tea_partition_generator::mkpart::Partgen;
use tea_partition_generator::os::Os;
use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

#[tokio::main]
async fn main() {
    let ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "btrfs".to_string(),
        true
    );

    // this number come from FE, use find_empty_space_sector_areav
    let partition_generator_ctx = TeaPartitionGenerator::new("/dev/sdb".to_string());
    let (start, end) = partition_generator_ctx.find_empty_space_sector_area();

    let ret = ctx.getresult(start, end);
    println!("partition design: {:#?}", ret);

    // return;

    if let Ok(ret_val) = ret {
        Partgen::do_dangerous_task_on(
            ret_val.clone(), ret_val.clone().install_method
        );

        let fstab = Os::append_swap_fstab(&ret_val.clone());
    }


}
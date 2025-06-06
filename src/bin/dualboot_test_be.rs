use tea_arch_chroot_lib::resource::MethodKind;
use tea_partition_generator::blueprint::Partition;
use tea_partition_generator::dual_boot_blockdev::{DualBootBlockdevice, DualbootBlkstuff};
use tea_partition_generator::mkpart::Partgen;
use tea_partition_generator::os::Os;
use tea_partition_generator::core::{PartitionGenerator, TeaPartitionGenerator};

#[tokio::main]
async fn main() {
    let mut ctx: DualbootBlkstuff = DualBootBlockdevice::blockdevice(
        "/dev/sdb".to_string(), 
        "ext4".to_string(),
        true
    );

    let disk = "/dev/sdb".to_string();

    // this number come from FE, use find_empty_space_sector_areav
    let partition_generator_ctx = TeaPartitionGenerator::new(disk.clone());
    let start = 10899456;
    let end = 115924991;

    // println!("partition design: {} {}", start, end);
    // return;
    if start == 0 && end == 0 {
        println!("no empty partition, aborting!");
        return;
    }

    let mut ret = ctx.getresult(start, end);

    // println!("runn {:#?}", ret);

    if let Ok(ret_val) = ret {
        Os::mkdisk_uninitalized(start, end, disk.clone());

        Partgen::do_dangerous_task_on(
            ret_val.clone(), ret_val.clone().install_method
        );

        // let fstab = Os::append_swap_fstab(&ret_val.clone());
    }


}
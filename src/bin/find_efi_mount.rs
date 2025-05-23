use tea_partition_generator::dual_boot_efi_mount;
fn main() {
    let ret = dual_boot_efi_mount::dualboot_efi_mount_open("/dev/sdb".to_string());

    println!("{:?}", ret);
}
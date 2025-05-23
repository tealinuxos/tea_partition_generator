use tea_partition_generator::os::Os;
fn main() {
    let ret = Os::get_efi_blockdevice("/dev/sdb".to_string());

    println!("{:?}", ret);
}
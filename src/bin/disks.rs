use tea_partition_generator::os;

fn main() {
    let ret = os::Os::get_disk_model("/dev/sdb".to_string());
    println!("{:#?}", ret);
}
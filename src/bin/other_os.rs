use tea_partition_generator::os::Os;
fn main() {
    let ret = Os::get_other_os();

    println!("{:?}", ret);
}
use tea_partition_generator::os;

fn main() {
    let ret = os::Os::get_other_os();

    println!("{:?}", ret);
}
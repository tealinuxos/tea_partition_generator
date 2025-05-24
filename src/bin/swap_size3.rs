use tea_partition_generator::os;

fn main() {
    let ret = os::Os::decide_swap_size3();

    println!("{:?}", ret);
}
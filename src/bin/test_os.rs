use tea_partition_generator::os;

#[tokio::main]
async fn main() {
    let ret = os::Os::get_other_os().await;

    println!("{:?}", ret);
}
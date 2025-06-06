use tea_partition_generator::disk_helper;

fn main() {
    let retq = disk_helper::nvme_split("/dev/nvme0n1p1".to_string());
    

    println!("{:?}", retq);
}
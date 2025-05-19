use tea_partition_generator::os::{DiskPredictor, StateDiskPredictor};

fn main() {
    let mut ret: StateDiskPredictor = DiskPredictor::new(
        "/dev/sdb".to_string(), 
        "gpt".to_string()
    );

    // ret._debug();

    let mut retq = ret.predict_next_disks_num();
    ret.mark(9); // 2 has been used
    let mut retq = ret.predict_next_disks_num();
    ret.mark(10); // 3 has been used
    // let mut retq = ret.predict_next_disks_num();
    let mut retq = ret.predict_next_disks_num();
    println!("{:?}", retq);



}
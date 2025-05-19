use tea_partition_generator::os::{DiskPredictor, StateDiskPredictor};

fn main() {
    let mut ret: StateDiskPredictor = DiskPredictor::new(
        "/dev/sdb".to_string(), 
        "mbr".to_string()
    );

    let mut retq = ret.predict_next_disks_num();
    ret.mark(2); // 2 has been used
    let mut retq = ret.predict_next_disks_num();




    println!("{:?}", retq);
}
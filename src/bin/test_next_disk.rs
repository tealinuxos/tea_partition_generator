use tea_partition_generator::os::{DiskPredictor, StateDiskPredictor};

fn main() {
    let mut disk_predictor = StateDiskPredictor::new(
        "/dev/sdb".to_string()
    );

    if let Ok(mut disk_predictor_val) = disk_predictor {
        let mut retq = disk_predictor_val.predict_next_disks_num();
        disk_predictor_val.mark(9); // 2 has been used
        let mut retq = disk_predictor_val.predict_next_disks_num();
        disk_predictor_val.mark(10); // 3 has been used
        // let mut retq = ret.predict_next_disks_num();
        let mut retq = disk_predictor_val.predict_next_disks_num();
        println!("{:?}", retq);
    }

    // ret._debug();




}
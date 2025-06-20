use tea_partition_generator::os::{DiskPredictor, StateDiskPredictor};

fn main() {
    let disk_predictor = StateDiskPredictor::new(
        "/dev/sdb".to_string()
    );

    if let Ok(mut disk_predictor_val) = disk_predictor {
        let mut _retq = disk_predictor_val.predict_next_disks_num();
        disk_predictor_val.mark(1); // 3 has been used
        let mut _retq = disk_predictor_val.predict_next_disks_num();
        disk_predictor_val.ignore(2);
        let mut _retq = disk_predictor_val.predict_next_disks_num();
        disk_predictor_val.mark(2);

        let mut _retq = disk_predictor_val.predict_next_disks_num();
        println!("{:?}", _retq);
    }

    // ret._debug();




}
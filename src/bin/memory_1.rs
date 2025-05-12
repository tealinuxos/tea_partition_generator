use tea_partition_generator::os;
use std::str::FromStr;
use duct::cmd;
use tea_partition_generator::disk_helper;

fn decide_swap_size(device: String) -> Option<u64> {
    let memory = os::Os::get_total_memory();

    let ideal_size = match memory {
        m if m < 8192 => m * 2,
        m if m < 16384 => ((m as f64 * 1.5) as usize).try_into().unwrap(),
        m if m < 32768 => m,
        m if m >= 32768 => m / 2,
        _ => memory,
    };

    let data = cmd!("blockdev", "--getsize64", device).read();
    // println!("{:#?}", data);

    if let Ok(data_val) = data {
        let ret = u64::from_str(&data_val).unwrap();
        let ret_mb = disk_helper::bytes2mb(ret) as f64;
        let mem_upper_limit = (ret_mb as f64)  * (32.0 / 100.0);

        if ideal_size as f64 > mem_upper_limit {
            // println!("max swap : {}", mem_upper_limit);
            Some(mem_upper_limit as u64)
        } else {
            // println!("max swap : {}", ideal_size);
            Some(ideal_size as u64)
        }
    } else {
        None
    }
}

fn main() {
    
    let ret = decide_swap_size("/dev/sdb".to_string());
    println!("{:#?}", ret);
}

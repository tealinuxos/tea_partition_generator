use regex::Regex;
use std::path::Path;

pub fn nvme_split(device_str: String) -> Option<(String, String)> {
    let re = Regex::new(r"^(/dev/nvme\d+n\d+)(p\d+)$").unwrap();

    if let Some(caps) = re.captures(&device_str) {
        let device = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let partition = caps.get(2).map_or("", |m| m.as_str()).to_string();
        Some((device, partition))
    } else {
        None
    }
}


pub fn scsi_split(device: String) -> Option<String> {
    if device.starts_with("/dev/sd") {
        let base = &device[..device.len() - 1];
        Some(base.to_string())
    } else {
        None
    }
}


pub fn gb2sector(x: u64, sector_size: u64) -> u64 {
    if sector_size == 0 {
        // this is probably non root user
        return 0;
    }
    (x * 1024 * 1024 * 1024) / sector_size
}

pub fn mb2sector(x: u64, sector_size: u64) -> u64 {
    if sector_size == 0 {
        // this is probably non root user
        return 0;
    }
    (x * 1024 * 1024) / sector_size
}

pub fn bytes2mb(x: u64) -> u64 {
    return x / 1024 / 1024;
}

pub fn disk_split_no_dev(input: String) -> String {

    let basename = Path::new(&input)
        .file_name()
        .and_then(|s| s.to_str()) 
        .unwrap();

    return basename.to_string();

}

pub fn remove_end_s(input: String) -> String {
    if input.ends_with('s') {
        input[..input.len() - 1].to_string()
    } else {
        input.to_string()
    }
}

pub fn format_size_human(size: u64) -> String {
    // WARNING: this func take bytes len as input

    if size < (1 * 1024 * 1024) {
        return "1M".to_string()
    } else if size < (999 * 1024 * 1024 * 1024) {
        return format!("{}G", size / 1024 / 1024 / 1024);
    } else if size >= (999 * 1024 * 1024 * 1024) {
        return format!("{}T", size / 1024 / 1024 / 1024);
    } else {
        return "very big".to_string(); // unlikely taken
    }
}
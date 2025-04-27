use regex::Regex;

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

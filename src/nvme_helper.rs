fn nvme_split(device_str: &str) -> Option<(String, String)> {
    let re = Regex::new(r"^(/dev/nvme\d+n\d+)(p\d+)$").unwrap();

    if let Some(caps) = re.captures(device_str) {
        let device = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let partition = caps.get(2).map_or("", |m| m.as_str()).to_string();
        Some((device, partition))
    } else {
        None
    }
}

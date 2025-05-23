use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<BlockDevice>,
}

#[derive(Debug, Deserialize)]
struct BlockDevice {
    name: String,
    pttype: Option<String>,
}

fn detect_partition_table(disk: &str) -> Option<String> {
    // Run `lsblk -J -o NAME,PTTYPE /dev/sdX`
    let output = Command::new("lsblk")
        .args(&["-J", "-o", "NAME,PTTYPE", disk])
        .output()
        .expect("failed to run lsblk");

    if !output.status.success() {
        eprintln!("lsblk error: {:?}", String::from_utf8_lossy(&output.stderr));
        return None;
    }

    let json = String::from_utf8_lossy(&output.stdout);
    let parsed: LsblkOutput = serde_json::from_str(&json).ok()?;

    // Find the root device (should match disk name, e.g., sdb)
    let disk_name = disk.strip_prefix("/dev/").unwrap_or(disk);
    for dev in parsed.blockdevices {
        if dev.name == disk_name {
            if let Some(ptable) = dev.pttype {
                if ptable == "dos" {
                    return Some("mbr".to_string());
                } else {
                    return Some(ptable);
                }
            }
        }
    }

    None
}

fn main() {
    let ret = detect_partition_table("/dev/sdb");

    println!("{:?}", ret);
}

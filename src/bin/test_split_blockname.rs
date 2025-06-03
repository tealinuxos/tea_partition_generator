use std::path::Path;

fn main() {
    let path = "/dev/nvme0n1p1";
    let basename = Path::new(path)
        .file_name()
        .and_then(|s| s.to_str()) 
        .unwrap();

    println!("{}", basename); // "sdb"

}
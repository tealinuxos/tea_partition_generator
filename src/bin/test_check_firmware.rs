use std::fs;

fn main() {
    let ret = fs::exists("/sys/firmware/efi");

    println!("{:?}", ret);
}
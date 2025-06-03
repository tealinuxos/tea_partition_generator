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

fn main() {
    let ret = format_size_human(35000320 * 512);

    println!("{}", ret);
}
use tea_partition_generator::os::Os;

fn main() {
    Os::mkdisk_uninitalized(93286400, 109936640, "/dev/sdb".to_string());
}
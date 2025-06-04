use tea_partition_generator::os::Os;

fn main() {
    Os::mkdisk_uninitalized(92573696, 167770111, "/dev/sdb".to_string());
}
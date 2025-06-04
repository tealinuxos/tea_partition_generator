use tea_partition_generator::os::Os;

fn main() {
    Os::mkdisk_uninitalized(60063744, 115857407, "/dev/sdb".to_string());
}
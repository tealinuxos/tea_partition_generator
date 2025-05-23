
use tea_partition_generator::os::Os;

fn main() {
    let ret = Os::patch_grub_config_disable_os_probe(false);
}
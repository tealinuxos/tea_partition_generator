use tea_partition_generator::single_boot_blockdev::{Blkstuff, SingleBootBlockdevice};

fn main() {
    let ctx: Blkstuff = SingleBootBlockdevice::blockdevice(
        String::from("/dev/sdb"),
        String::from("ext4"),
        String::from("mbr"),
        false,
    );

    let ret = ctx.getresult();

    println!("{:#?}", ret);
}

use tea_partition_generator::parted_parser;

fn main() {
    let test_string = "BYT;
/dev/sdb:50331648s:scsi:512:512:gpt:ATA QEMU HARDDISK:;
1:34s:2047s:2014s:free;
1:2048s:12584959s:12582912s:ext4:randomstr:boot, esp;
1:12584960s:33722367s:21137408s:free;
2:33722368s:50329599s:16607232s:btrfs::;
1:50329600s:50331614s:2015s:free;".to_string();

    let ret = parted_parser::PartedResult::parse(test_string);

    println!("{:#?}", ret);
}
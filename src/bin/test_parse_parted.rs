use tea_partition_generator::parted_parser;

fn main() {
    let test_string = "BYT;
/dev/sdd:500118192s:scsi:512:512:gpt:ATA KYO-256GB:;
1:34s:2047s:2014s:free;
1:2048s:2099199s:2097152s:fat32::boot, esp;
2:2099200s:231682047s:229582848s:btrfs::;
3:231682048s:365897727s:134215680s:ntfs:Basic data partition:msftdata;
4:365897728s:365930495s:32768s::Microsoft reserved partition:msftres, no_automount;
5:365930496s:500117503s:134187008s:ntfs:Basic data partition:msftdata;
1:500117504s:500118158s:655s:free;".to_string();

    let ret = parted_parser::PartedResult::parse(test_string);

    println!("{:#?}", ret);
}
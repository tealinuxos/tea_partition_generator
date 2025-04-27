#[derive(Default)]
#[derive(Debug)]
pub struct PartedDiskInfo {
    pub model: String,
    pub disk_type: String,
    pub disk: String,
    pub sector: u64,
    pub sector_size_logical: u32,
    pub sector_size_physical: u32,
    pub partition_table: String,
    pub disk_flags: String,
}

#[derive(Default)]
#[derive(Debug)]
pub struct PartedData {
    pub number: u32,
    pub start: u64,
    pub end: u64,
    pub size: u64,
    pub fs: String,
    pub partition_name: String,
    pub flags: String,
}

#[derive(Default)]
#[derive(Debug)]
pub struct PartedResult {
    pub info: PartedDiskInfo,
    pub data: Vec<PartedData>,
}

impl PartedResult {
    fn _gparted_disk_parse(data: String) -> PartedDiskInfo {
        let mut buf = PartedDiskInfo::default();

        let raw_str = data.split(":");
        let mut iter_counter = 0;

        for raw_str_i in raw_str {
            match iter_counter {
                0 => {
                    buf.disk = raw_str_i.to_string().clone()
                },
                1 => {
                    let mut temp = raw_str_i.to_string();
                    // let temp2: &str = temp.pop();
                    // buf.sector = 
                    temp.pop();
                    buf.sector = temp.parse::<u64>().unwrap();
                },
                2 => {
                    buf.disk_type = raw_str_i.to_string().clone()
                },
                3 => {
                    let temp = raw_str_i.to_string().clone();
                    buf.sector_size_logical = temp.parse::<u32>().unwrap()
                },
                4 => {
                    let temp = raw_str_i.to_string().clone();
                    buf.sector_size_physical = temp.parse::<u32>().unwrap()
                },
                5 => {
                    buf.partition_table = raw_str_i.to_string().clone();
                },
                6 => {
                    buf.model = raw_str_i.to_string().clone();
                },
                7 => {
                    buf.disk_flags = raw_str_i.to_string().clone();
                },
                
                _ => {}
            }
            // println!("{:?}", raw_str_i);
            iter_counter+=1;
        }

        // println!("{:?}", buf);
        buf
    }

    fn _gparted_disk_partition_parse(data: String)  -> PartedData {
        let mut buf = PartedData::default();

        let raw_str = data.split(":");
        let mut iter_counter = 0;

        for raw_str_i in raw_str {
            match iter_counter {
                0 => {
                    let temp = raw_str_i.to_string();
                    buf.number = temp.parse::<u32>().unwrap();
                },
                1 => {
                    let mut temp = raw_str_i.to_string();
                    temp.pop();
                    buf.start = temp.parse::<u64>().unwrap();
                },
                2 => {
                    let mut temp = raw_str_i.to_string();
                    temp.pop();
                    buf.end = temp.parse::<u64>().unwrap();
                },
                3 => {
                    let mut temp = raw_str_i.to_string();
                    temp.pop();
                    buf.size = temp.parse::<u64>().unwrap();
                },
                4 => {
                    buf.fs = raw_str_i.to_string().clone();
                },
                5 => {
                    buf.partition_name = raw_str_i.to_string().clone();
                },
                6 => {
                    buf.flags = raw_str_i.to_string().clone();
                }
                _ => {}
            }
            // println!("{:?}", raw_str_i);
            iter_counter+=1;
        }

        // println!("{:?}", buf);
        buf
    }

    pub fn parse(raw_string: String) -> PartedResult {
        let raw_str_semicolon_split = raw_string.split(";");
        let mut iter_counter = 0;

        let mut parted_data_buf: Vec<PartedData> = Vec::new();
        let mut parted_result_buf = PartedResult::default();

        for raw_str_semicolon_split_i in raw_str_semicolon_split {
            let sanitized_raw_str = raw_str_semicolon_split_i.trim_start_matches('\n');
            // println!("{:?}", sanitized_raw_str);

            if sanitized_raw_str == "" {
                continue;
            }

            if iter_counter == 0 {
                // skip, BYT
            } 
            if iter_counter == 1 {
                // parse disk info
                parted_result_buf.info = Self::_gparted_disk_parse(sanitized_raw_str.to_string().clone());
                // println!("{:?}", );
            } 

            // debug
            if iter_counter > 1 {
                // println!("num: {}", iter_counter);
                let ret = Self::_gparted_disk_partition_parse(sanitized_raw_str.to_string().clone());
                parted_data_buf.push(ret);
            }
            iter_counter+=1;
        }

        parted_result_buf.data = parted_data_buf;

        parted_result_buf
    }
}

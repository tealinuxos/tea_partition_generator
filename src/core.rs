use crate::config;
use crate::os;
use crate::disk_helper;
use crate::parted_parser;
use duct::cmd;
pub struct TeaPartitionGenerator {
    selected: String,
}

// this struct bring such
// /dev/sdb instead partition like /dev/sdb3 
// whatever it is

#[derive(Debug)]
pub struct OsOnDisk
{
    pub name: String,
    pub path: String
}

#[derive(Debug)]
pub struct EmptySpace {
    pub number: i32,    // this number is generated sequentially though vector push, not from parted
    pub start: u64,
    pub end: u64,
}

#[derive(Debug)]
pub struct ListsAllSpace {
    pub index: u32,
    pub partition_name: String,
    pub start: u64,
    pub end: u64,
    pub size_h: String, // human version for FE
    pub fs: Option<String>, // FIXME: add enum
}

impl TeaPartitionGenerator {
    pub fn new(selected: String) -> TeaPartitionGenerator {
        TeaPartitionGenerator { selected }
    }

    fn _os_probe_output_find_other_os(&self, data: &Vec<os::Os>) -> bool {
        let mut found = false;
        for data_i in data {
            if data_i.path.starts_with("/dev/nvme") {
                // handle nvme
                let ret = disk_helper::nvme_split(data_i.path.clone());
                if let Some(ret_val) = ret {
                    if ret_val.0 == self.selected {
                        found = true;
                    }
                }
            } else if data_i.path.starts_with("/dev/sd") { // scsi 

                let ret = disk_helper::scsi_split(data_i.path.clone());
                if let Some(ret_val) = ret {
                    if ret_val == self.selected {
                        found = true;
                    }
                }
            }
        }
        found
    }
}

pub trait PartitionGenerator {
    fn new(selected: String) -> TeaPartitionGenerator;
    fn has_other_os(&self) -> bool;
    fn disk_list_other_os() -> Option<Vec<OsOnDisk>>;
    fn find_empty_space_sector_area(&self) -> (u64, u64);
    fn find_empty_space_sector_areav(&self) -> Vec<EmptySpace>;
    fn find_partition_sector_areav(&self) -> ();
}

impl PartitionGenerator for TeaPartitionGenerator {
    fn new(selected: String) -> TeaPartitionGenerator {
        TeaPartitionGenerator {
            selected
        }
    }

    fn has_other_os(&self) -> bool {
        let ret = os::Os::get_other_os();

        if let Ok(ret_val) = ret {
            if let Some(data) = ret_val {

                // check if osprobe output contain selected disks
                // for data_i in &data {
                //     println!("{:?}", data_i.path );
                //     // if 
                // }
                // _parse_os_probe_match(&data);
                let ret = self._os_probe_output_find_other_os(&data);
                // println!("fn {:?}", ret);
                ret
            } else {
                false
            }
        } else {
            false
        }
    }

    // convention: start ~ end
    // DEPRECATED
    fn find_empty_space_sector_area(&self) -> (u64, u64) {
        // the disk must be larger than 7 GiB (currently)
        // let run = format!(, self.selected);
        let parted = cmd!("sudo", "parted", "-m", self.selected.clone(), "unit", "s", "print", "free").read();

        if let Ok(parted_data) = parted {
            let ret = parted_parser::PartedResult::parse(parted_data);

            for parted_data_i in &ret.data {
                // NOTE: Tunning this number
                if ((ret.info.sector_size_logical as u64) * parted_data_i.size) > (config::MINIMUM_DISK_SIZE * 1024 * 1024 * 1024) && parted_data_i.fs == "free" {
                    let mut start = parted_data_i.start;
                    if start < 2048 {
                        start = 2049; // pad
                    }
                    
                    return (start, parted_data_i.end)
                }
            }

            // println!("{:#?}", ret);
        }

        (0,0)
    }


    fn find_empty_space_sector_areav(&self) -> Vec<EmptySpace> {
        // the disk must be larger than 7 GiB (currently)
        // let run = format!(, self.selected);
        let mut buf: Vec<EmptySpace> = Vec::new();
        let parted = cmd!("sudo", "parted", "-m", self.selected.clone(), "unit", "s", "print", "free").read();

        if let Ok(parted_data) = parted {
            let ret = parted_parser::PartedResult::parse(parted_data);

            let mut n = 1;

            for parted_data_i in &ret.data {
                // NOTE: Tunning this number, x * (1024 * 1024 * 1024)

                if ((ret.info.sector_size_logical as u64) * parted_data_i.size) > 7516192768 && parted_data_i.fs == "free" {
                    // return (parted_data_i.start, parted_data_i.end)
                    // buf.push((parted_data_i.start, parted_data_i.end))
                    buf.push(EmptySpace {
                        number: n,
                        start: parted_data_i.start,
                        end: parted_data_i.end,
                    });
                    n = n + 1;
                }
            }

            // println!("{:#?}", ret);
        }

        buf
    }

    fn find_partition_sector_areav(&self) -> () {
        // the disk must be larger than X GiB (see config.rs please)

        let mut buf: Vec<ListsAllSpace> = Vec::new();

        let parted = cmd!("sudo", "parted", "-j", self.selected.clone(), "unit", "s", "print", "free").read();

        if let Ok(parted_val) = parted {
            let v: serde_json::Value = serde_json::from_str(&parted_val).unwrap();
            let partition_vec = v["disk"]["partitions"].as_array();

            let n = 0;
            
            let disk_name = disk_helper::disk_split_no_dev(v["disk"]["path"].as_str().unwrap().to_string());
            let sector_size = v["disk"]["physical-sector-size"].as_i64().unwrap() as u64;

            for parted_data_i in partition_vec.unwrap() {
                // println!("{:#?}", parted_data_i);
                let trimmed_start = disk_helper::remove_end_s(parted_data_i["start"].as_str().unwrap().to_string());
                let trimmed_end = disk_helper::remove_end_s(parted_data_i["end"].as_str().unwrap().to_string());
                
                let _fs: Option<String> = match parted_data_i["filesystem"].as_str() {
                    Some(data) => Some(data.to_string()),
                    Option::None => Some("UNALLOCATED".to_string())
                };

                let mut _partition_name: String = "Unallocated".to_string();
                if parted_data_i["number"].as_i64().unwrap() != 0 {
                    _partition_name = format!("{}{}", disk_name, parted_data_i["number"].as_i64().unwrap());
                }
                
                let _int_start = trimmed_start.parse::<u64>().unwrap();
                let _int_end = trimmed_end.parse::<u64>().unwrap() as u64;

                buf.push(ListsAllSpace {
                    index: n,
                    partition_name: _partition_name,
                    start: _int_start,
                    end: _int_end,
                    // size_h: format!("{}"), // test
                    size_h: disk_helper::format_size_human((_int_end - _int_start) * sector_size),
                    fs: _fs
                })
            }

            println!("{:#?}", buf);

            return 
        }
    }

    // this func return 
    // 
    // example:
    // /dev/sdb instead /dev/sdb3 (in os prober output)
    // and make sure if the os prober output is match with current partition layout
    fn disk_list_other_os() -> Option<Vec<OsOnDisk>> {
        let ret = os::Os::get_other_os();

        let mut buf: Vec<OsOnDisk> = Vec::new();

        if let Ok(ret_val) = ret {
            if let Some(data) = ret_val {
                for data_i in data {
                    if data_i.path.starts_with("/dev/nvme") {
                        // handle nvme
                        let ret = disk_helper::nvme_split(data_i.path.clone());
                        if let Some(ret_val) = ret {
                            buf.push(OsOnDisk {
                                path: ret_val.0,
                                name: data_i.name
                            });

                        }
                    } else if data_i.path.starts_with("/dev/sd") { // scsi 
        
                        let ret = disk_helper::scsi_split(data_i.path.clone());
                        if let Some(ret_val) = ret {
                            buf.push(OsOnDisk {
                                path: ret_val,
                                name: data_i.name
                            });
                        }
                    }
                }
            }
        }

        if buf.len() == 0 {
            None
        } else {
            Some(buf)
        }
    }
     
}

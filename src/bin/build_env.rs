use std::env;

#[derive(Debug)]
pub enum BuildType {
    Production,
    Dev
}

pub fn tealinux_build_env() -> Result<BuildType, ()> {
    match option_env!("TEALINUX_BUILD") {
        Some(val) => {
            if val == "production" {
                return Ok(BuildType::Production);
            } else if val == "prod" {
                return Ok(BuildType::Production);
            } else if val == "dev" {
                return Ok(BuildType::Dev);
            } else {
                println!("Error TEALINUX_BUILD is not set properly, see https://github.com/tealinuxos/tealinux-installer?tab=readme-ov-file#building please");
            return Err(());
            }
        }
        None => {
            println!("Error TEALINUX_BUILD not found during build, see https://github.com/tealinuxos/tealinux-installer?tab=readme-ov-file#building please");
            return Err(());
        },
    }
}

fn main() {
    let ret = self::tealinux_build_env();
    println!("ret {:?}", ret.unwrap()); // this is safe to unwrap

}
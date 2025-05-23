// GOAL

// find where efi partition is, i.e: /dev/sda1
// mount it as /tealinux-mount/boot/efi 
// seq: 
//      - mount root first
//      - mkdir /boot/efi
//      - mount /dev/sda1 as /tealinux-mount/boot/efi
//      - SO when this code [https://github.com/tealinuxos/tealinux-installer/blob/cd3ae4e28f5a63a25ded2294aeb5b55b9ac53748/src-tauri/src/installer/step/bootloader.rs#L23] 
//        doing its job, there was already mounted and go!, no additional mount required
//      - firmware path is guarantee not none

use crate::mounting::Mount;
use crate::os;
use crate::mounting;

pub fn dualboot_efi_mount_open(device: String) {
    let blockdev_num = os::Os::get_efi_blockdevice(device.clone());
    if let Ok(blockdev_num_val) = blockdev_num {
        let fullfilled_path = format!("{}{}", device.clone(), blockdev_num_val);

        println!("opening efi mountpoint for {}", fullfilled_path);
        mounting::MountPoint::run_mount_for(fullfilled_path, "/tealinux-mount/boot/efi".to_string(), None);
    } else {
        println!("dualboot efi mount: something went wrong"); // FIXME: add modal that can be fired
    }
}
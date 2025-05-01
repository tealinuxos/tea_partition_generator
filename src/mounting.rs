

use crate::blueprint::{Storage, Partition};
use duct::cmd;
use std::io::Error;

#[derive(Debug)]
pub struct MountPoint {
    data: Storage
}

pub trait Mount {
    fn new(data: Storage) -> Self;
    fn mount_all(&self);
}

impl MountPoint {
    fn _umount_all() {
        let _ = cmd!("umount", "--recursive", "/tealinux-mount").run();
    }

    fn _gen_mountpoint_strformat(data: String) -> String {
        format!("/tealinux-mount{}", data)
    }

    fn mount(partition_path: &String, mountpoint: &String, options: Option<Vec<&str>>) -> Result<(), Error> {
        let options = {
            if let Some(options) = options {
                let options: String = options
                    .iter()
                    .map(|o| format!("{},", o))
                    .collect();

                Some(options)
            } else {
                None
            }
        };

        if options.is_none() {
            cmd!("mount", partition_path, mountpoint).run()?;
        } else {
            cmd!("mount", "-o", options.unwrap(), partition_path, mountpoint).run()?;
        }

        Ok(())
    }

    fn umount(path: String) {
        let _ = cmd!("umount", path).run();
    }

    fn mkdir_force(dir: String) {
        let _ = cmd!("mkdir", "--parents", dir).run();
    }

    fn _handle_btrfs_mount(data: &Partition) -> i32 {
        if let Some(path) = &data.path {
            let _ = Self::mount(
                &path, 
                &Self::_gen_mountpoint_strformat("".to_string()), 
                None);
            
            let btrfs_mnt_root = &Self::_gen_mountpoint_strformat("/@".to_string());
            let _ = cmd!("btrfs", "subvolume", "create", btrfs_mnt_root).run();

            let btrfs_mnt_root = &Self::_gen_mountpoint_strformat("/@home".to_string());
            let _ = cmd!("btrfs", "subvolume", "create", btrfs_mnt_root).run();

            let btrfs_mnt_root = &Self::_gen_mountpoint_strformat("".to_string());
            let _ = cmd!("btrfs", "subvolume", "list", btrfs_mnt_root).run();

            Self::umount(btrfs_mnt_root.to_string());
            

            let btrfs_mount_opt = vec!["subvol=@"];
            let _ = Self::mount(
                &path, 
                &Self::_gen_mountpoint_strformat("".to_string()), 
                Some(btrfs_mount_opt));
            
            let _ = cmd!("mkdir", "/tealinux-mount/home").run();

            let btrfs_mount_opt = vec!["subvol=@home"];
            let _ = Self::mount(
                    &path, 
                    &Self::_gen_mountpoint_strformat("/home".to_string()), 
                    Some(btrfs_mount_opt));

        } else {
            return -1;
        }

        return 0;

    }

    fn _handle_ext4_mount(data: &Partition) -> i32 {
        if let Some(path) = &data.path {
            let _ = Self::mount(
                &path, 
                &Self::_gen_mountpoint_strformat("".to_string()), 
                None);
        } else {
            return -1;
        }

        return 0;

    }
}

impl Mount for MountPoint {
    fn new(data: Storage) -> Self {
        MountPoint { 
            data: data
        }
    }

    fn mount_all(&self) {
        Self::_umount_all();

        if let Some(partitions_val) = &self.data.partitions {

            for partitions_val_i in partitions_val {
                if let Some(data) = &partitions_val_i.mountpoint {
                    println!("path -> {}", data.clone());
                    if data == "/boot/efi" {
                        let local_mount = Self::_gen_mountpoint_strformat(data.clone());
                        println!("running mkdir --parent {}", local_mount);
                        Self::mkdir_force(local_mount.clone());

                        if let Some(path_data) = &partitions_val_i.path {
                            let _ = Self::mount(path_data, &local_mount.clone(), None);
                        }
                    }

                    if data == "/" {
                        let local_mount = Self::_gen_mountpoint_strformat(data.clone());
                        println!("running mkdir --parent {}", local_mount);
                        Self::mkdir_force(local_mount.clone());

                        if partitions_val_i.filesystem.as_deref() == Some("btrfs") {
                            Self::_handle_btrfs_mount(partitions_val_i);
                        }

                        if partitions_val_i.filesystem.as_deref() == Some("ext4") {
                            Self::_handle_ext4_mount(partitions_val_i);
                        }

                    }


                }
            }
        }
    }
}
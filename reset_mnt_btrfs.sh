sudo umount /tealinux-mount/home
sudo umount /tealinux-mount
btrfs subvolume delete /tealinux-mount/@home
btrfs subvolume delete /tealinux-mount/@
sudo umount /tealinux-mount/boot/efi
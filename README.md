### Setup
```
curl https://sh.rustup.rs -sSf | sh
rustup override add nightly
rustup component add llvm-tools-preview
cargo install cargo-xbuild
rustup component add rust-src
cargo xbuild --target x86_64-redleaf.json
```


### Prerequisites
* Install Qemu

```
sudo apt-get install qemu
```

* Install nasm

```
sudo apt-get install nasm
```

* Install Grub

```
apt-get install grub-pc-bin
```

* Install dependencies to compile rumpkernel

```
apt-get install zlib1g-dev make gcc build-essential
```

### Run

```
make run
```

If you see complains from the `grub-mkrescue`, install the `xorriso` package
```
grub-mkrescue: warning: Your xorriso doesn't support `--grub2-boot-info'. Some features are disabled. Please use xorriso 1.2.9 or later..                                                 
```

### Boot on baremetal from a USB drive

Copy the ISO disk image to the USB stick (make sure to use correct device for the 
USB drive, otherwise you can overwrite your hard disk). You can use lsblk on Ubuntu
to list block devices

```
lsblk
```

For me it's /dev/sda or /dev/sdb but my laptop runs off an NVMe device, so for you 
/dev/sda may very well be your root device, not a USB!

```
sudo dd if=build/os.iso of=/dev/<your_usb_drive> bs=1MB
sync
```

### Boot on baremetal from a Linux partition

``` 
sudo cp build/kernel.bin /boot/
```
Add the following entry to the grub menu list. On a Linux machine this can
be done by adding this to the /etc/grub.d/40_custom. You might adjust the
root='hd0,2' to reflect where your Linux root is on disk, e.g., maybe it's on
root='hd0,1'

```
set timeout=30
menuentry "RedLeaf" {
    insmod ext2
    set root='hd0,1'
    set kernel='/boot/kernel.bin'
    echo "Loading ${kernel}..."
    multiboot2 ${kernel} ${kernel}
    boot
}
```

Update grub

```
  sudo sudo update-grub2
```

Reboot and choose the "RedLeaf" entry. Make sure that you can see the grub menu
list by editing /etc/default/grub making sure that GRUB_HIDDEN_TIMEOUT_QUIET is
set to "false". 

```
  GRUB_HIDDEN_TIMEOUT_QUIET=false
```

### Notes

The baremetal Rust setup (features, linking, etc. is best describe in https://os.phil-opp.com/set-up-rust/).

A cleaner baremental setup (multi-boot and no dependencies on external tools): https://kernelstack.net/2019-07-13-rust-os-1/

Two versions of Philipp Opper blog: https://os.phil-opp.com (v2) and https://os.phil-opp.com/first-edition/ (v1)

Naked functions for exceptions: https://os.phil-opp.com/first-edition/extra/naked-exceptions/

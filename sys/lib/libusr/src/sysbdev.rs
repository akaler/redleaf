extern crate alloc;
use spin::Once;
use alloc::boxed::Box;
use rref::RRef;
use usr::bdev::BDev;

pub static BDEV: Once<Box<dyn BDev + Sync + Send>> = Once::new();

pub fn init(bdev: Box<dyn BDev + Sync + Send>) {
    BDEV.call_once(|| bdev);
}

pub fn sys_read(block: u32, data: &mut RRef<[u8; 512]>) {
    let bdev = BDEV.r#try().expect("BDev interface is not initialized.");
    bdev.read(block, data)
}

pub fn sys_write(block: u32, data: &[u8; 512]) {
    let bdev = BDEV.r#try().expect("BDev interface is not initialized.");
    bdev.write(block, data)
}

#![no_std]
#![feature(associated_type_defaults)]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::collections::VecDeque;

use spin::{MutexGuard};
use core::alloc::Layout;


use pci_driver::PciClass;

pub mod errors;

pub trait Syscall {
    fn sys_print(&self, s: &str);
    fn sys_println(&self, s: &str);
    fn sys_cpuid(&self) -> u32;
    fn sys_yield(&self);
    fn sys_create_thread(&self, name: &str, func: extern fn()) -> Box<dyn Thread>;
    fn sys_current_thread(&self) -> Box<dyn Thread>;
    fn sys_get_current_domain_id(&self) -> u64;
    unsafe fn sys_update_current_domain_id(&self, new_domain_id: u64) -> u64;
    fn sys_alloc(&self) -> *mut u8;
    fn sys_free(&self, p: *mut u8);
    fn sys_alloc_huge(&self, sz: u64) -> *mut u8;
    fn sys_free_huge(&self, p: *mut u8);
    fn sys_backtrace(&self);
    fn sys_dummy(&self);
}

#[derive(Clone,Copy,Debug)]
pub enum ThreadState {
    Runnable = 1,
    Paused = 2,
    Waiting = 3, 
}

/// RedLeaf thread interface
pub trait Thread {
    fn get_id(&self) -> u64;
    fn set_affinity(&self, affinity: u64);
    fn set_priority(&self, prio: u64);
    fn set_state(&self, state: ThreadState);
    fn sleep(&self, guard: MutexGuard<()>);
}

/// RedLeaf PCI bus driver interface
pub trait PCI {
    fn pci_register_driver(&self, pci_driver: &mut dyn pci_driver::PciDriver, bar_index: usize, class: Option<(PciClass, u8)>) -> Result<(), ()>;
    /// Boxed trait objects cannot be cloned trivially!
    /// https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/6
    fn pci_clone(&self) -> Box<dyn PCI>;
}

/// RedLeaf network interface
pub trait Net {
    fn submit_and_poll(&mut self, packets: &mut VecDeque<Vec<u8>>, reap_queue: &mut VecDeque<Vec<u8>>, tx: bool) -> usize;
}

/// RedLeaf Domain interface
pub trait Domain {
    fn get_domain_id(&self) -> u64;
}

/// Shared heap interface
pub trait Heap {
    unsafe fn alloc(&self, domain_id: u64, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&self, domain_id: u64, ptr: *mut u8, layout: Layout);
    unsafe fn change_domain(&self, from_domain_id: u64, to_domain_id: u64, ptr: *mut u8, layout: Layout);
}

pub static IRQ_TIMER: u8 = 32;

pub trait Interrupt {
    // Recieve an interrupt
    fn sys_recv_int(&self, int: u8);
    fn int_clone(&self) -> Box<dyn Interrupt>;
}

pub trait PciResource {
    fn read(&self, bus: u8, dev: u8, func: u8, offset: u8) -> u32;
    fn write(&self, bus: u8, dev: u8, func: u8, offset: u8, value: u32);
}

pub trait PciBar {
    fn get_bar_region(&self, base: u64, size: usize,
                      pci_driver: pci_driver::PciDrivers) ->  pci_driver::BarRegions;

}

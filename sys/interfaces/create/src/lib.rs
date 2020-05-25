#![no_std]
#![feature(associated_type_defaults)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use syscalls::{Heap, Domain, Interrupt};
use usr::error::Result;

/* AB: XXX: first thing: change all names to create_domain -- it's absurd */
pub trait CreatePCI: Send + Sync {
    fn create_domain_pci(&self) -> (Box<dyn syscalls::Domain>, Box<dyn usr::pci::PCI>);
}

pub trait CreateAHCI: Send + Sync {
    fn create_domain_ahci(&self, pci: Box<dyn usr::pci::PCI>) -> (Box<dyn syscalls::Domain>, Box<dyn usr::bdev::BDev>);
}

pub trait CreateMemBDev: Send + Sync {
    fn create_domain_membdev(&self, memdisk: &'static mut [u8]) -> (Box<dyn syscalls::Domain>, Box<dyn usr::bdev::BDev>);
    fn recreate_domain_membdev(&self, dom: Box<dyn syscalls::Domain>, memdisk: &'static mut [u8]) -> (Box<dyn syscalls::Domain>, Box<dyn usr::bdev::BDev>);
}

pub trait CreateBDevShadow: Send + Sync {
    fn create_domain_bdev_shadow(&self, create: Arc<dyn CreateMemBDev>) -> (Box<dyn syscalls::Domain>, Box<dyn usr::bdev::BDev>);
}

pub trait CreateIxgbe: Send + Sync {
    fn create_domain_ixgbe(&self, pci: Box<dyn usr::pci::PCI>) -> (Box<dyn syscalls::Domain>, Box<dyn usr::net::Net + Send>);
}

pub trait CreateNetShadow: Send + Sync {
    fn create_domain_net_shadow(&self, create: Arc<dyn CreateIxgbe>, pci: Box<dyn usr::pci::PCI>) -> (Box<dyn syscalls::Domain>, Box<dyn usr::net::Net + Send>);
}

pub trait CreateNvme {
    fn create_domain_nvme(&self, pci: Box<dyn usr::pci::PCI>) -> Box<dyn syscalls::Domain>;
}

pub trait CreateXv6FS: Send + Sync {
    fn create_domain_xv6fs(&self, bdev: Box<dyn usr::bdev::BDev>) ->(Box<dyn syscalls::Domain>, Box<dyn usr::vfs::VFS + Send>);
}

pub trait CreateXv6Usr: Send + Sync {
    fn create_domain_xv6usr(&self, name: &str, xv6: Box<dyn usr::xv6::Xv6>, blob: &[u8], args: &str) -> Result<Box<dyn syscalls::Domain>>;
}
pub type CreateXv6UsrPtr = Box<dyn CreateXv6Usr + Send + Sync>;

pub trait CreateXv6: Send + Sync {
    fn create_domain_xv6kernel(&self,
                               ints: Box<dyn Interrupt>,
                               create_xv6fs: Arc<dyn CreateXv6FS>,
                               create_xv6usr: Arc<dyn CreateXv6Usr + Send + Sync>,
                               bdev: Box<dyn usr::bdev::BDev>,
                               net: Box<dyn usr::net::Net>) -> (Box<dyn syscalls::Domain>, Box<dyn usr::xv6::Xv6>);
}

pub trait CreateDomA: Send + Sync {
    fn create_domain_dom_a(&self) -> (Box<dyn syscalls::Domain>, Box<dyn usr::dom_a::DomA>);
}

pub trait CreateDomB: Send + Sync {
    fn create_domain_dom_b(&self, dom_a: Box<dyn usr::dom_a::DomA>) -> Box<dyn syscalls::Domain>;
}

pub trait CreateDomC: Send + Sync {
    fn create_domain_dom_c(&self) -> (Box<dyn syscalls::Domain>, Box<dyn usr::dom_c::DomC>);
    fn recreate_domain_dom_c(&self, dom: Box<dyn syscalls::Domain>) -> (Box<dyn syscalls::Domain>, Box<dyn usr::dom_c::DomC>);
}

pub trait CreateDomD: Send + Sync {
    fn create_domain_dom_d(&self, dom_c: Box<dyn usr::dom_c::DomC>) -> Box<dyn syscalls::Domain>;
}

pub trait CreateShadow: Send + Sync {
    fn create_domain_shadow(&self, create_dom_c: Arc<dyn CreateDomC>) -> (Box<dyn syscalls::Domain>, Box<dyn usr::dom_c::DomC>);
}

pub trait CreateBenchnet: Send + Sync {
    fn create_domain_benchnet(&self, net: Box<dyn usr::net::Net>) -> Box<dyn syscalls::Domain>;
}

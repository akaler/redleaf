#![no_std]
#![feature(associated_type_defaults)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use syscalls::{Heap, Domain, PCI, PciBar, PciResource, Net, Interrupt};
use usr::{bdev::BDev, vfs::VFS, xv6::Xv6, dom_a::DomA, dom_c::DomC};

pub trait CreatePCI {
    fn create_domain_pci(&self) -> (Box<dyn Domain>, Box<dyn PCI>);
}

pub trait CreateAHCI {
    fn create_domain_ahci(&self, pci: Box<dyn PCI>) -> (Box<dyn Domain>, Box<dyn BDev + Send + Sync>);
}

pub trait CreateMemBDev {
    fn create_domain_membdev(&self) -> (Box<dyn Domain>, Box<dyn BDev + Send + Sync>);
}

pub trait CreateIxgbe {
    fn create_domain_ixgbe(&self, pci: Box<dyn PCI>) -> (Box<dyn Domain>, Box<dyn Net>);
}

pub trait CreateXv6FS {
    fn create_domain_xv6fs(&self, bdev: Box<dyn BDev>) ->(Box<dyn Domain>, Box<dyn VFS + Send>);
}

pub trait CreateXv6Usr {
    fn create_domain_xv6usr(&self, name: &str, xv6: Box<dyn usr::xv6::Xv6>, blob: &[u8], args: &str) -> Result<Box<dyn syscalls::Domain>, &'static str>;
}
pub type CreateXv6UsrPtr = Box<dyn CreateXv6Usr + Send + Sync>;

pub trait CreateXv6 {
    fn create_domain_xv6kernel(&self,
                               ints: Box<dyn Interrupt>,
                               create_xv6fs: Arc<dyn CreateXv6FS>,
                               create_xv6usr: Arc<dyn CreateXv6Usr + Send + Sync>,
                               bdev: Box<dyn BDev + Send + Sync>) -> Box<dyn Domain>;
}

pub trait CreateDomA {
    fn create_domain_dom_a(&self) -> (Box<dyn Domain>, Box<dyn DomA>);
}

pub trait CreateDomB {
    fn create_domain_dom_b(&self, dom_a: Box<dyn DomA>) -> Box<dyn Domain>;
}

pub trait CreateDomC {
    fn create_domain_dom_c(&self) -> (Box<dyn Domain>, Box<dyn DomC>);
}

pub trait CreateDomD {
    fn create_domain_dom_d(&self, dom_c: Box<dyn DomC>) -> Box<dyn Domain>;
}

pub trait CreateShadow {
    fn create_domain_shadow(&self, dom_c: Box<dyn DomC>) -> (Box<dyn Domain>, Box<dyn DomC>);
}

#![no_std]
#![no_main]
#![feature(
    box_syntax,
)]
#![forbid(unsafe_code)]
extern crate malloc;
extern crate alloc;
use libsyscalls;
use syscalls::{Syscall, Heap};
use create;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::collections::VecDeque;
use console::println;
use core::alloc::Layout;
use core::panic::PanicInfo;
use usr;
use rref::{RRef, RRefDeque};
use alloc::vec::Vec;
use usr::error::Result;
use usr::bdev::{NvmeBDev, BlkReq};
use usr::pci::PCI;
use usr::rpc::RpcResult;
use create::CreateNvme;
use spin::Mutex;

struct ShadowInternal {
    create: Arc<dyn CreateNvme>,
    nvme: Box<dyn NvmeBDev>,
    dom: Option<Box<dyn syscalls::Domain>>,
}

impl ShadowInternal {
    fn new(create: Arc<dyn CreateNvme>, pci: Box<dyn PCI>) -> Self {
        let (dom, nvme) = create.create_domain_nvme(pci);
        Self {
            create,
            nvme,
            dom: Some(dom),
        }
    }
}

struct Shadow {
    shadow: Mutex<ShadowInternal>,
}

impl Shadow {
    fn new(create: Arc<dyn CreateNvme>, pci: Box<dyn PCI>) -> Self {
        Self {
            shadow: Mutex::new(ShadowInternal::new(create, pci)),
        }
    }
}

impl NvmeBDev for Shadow {
    fn submit_and_poll_rref(
    &self,
    submit: RRefDeque<BlkReq, 128>,
    collect: RRefDeque<BlkReq, 128>,
    write: bool,
    ) -> RpcResult<Result<(
        usize,
        RRefDeque<BlkReq, 128>,
        RRefDeque<BlkReq, 128>,
    )>> {
        self.shadow.lock().nvme.submit_and_poll_rref(submit, collect, write)
    }

    fn poll_rref(&mut self, collect: RRefDeque<BlkReq, 1024>) ->
            RpcResult<Result<(usize, RRefDeque<BlkReq, 1024>)>> {
        self.shadow.lock().nvme.poll_rref(collect)
    }

    fn get_stats(&mut self) -> RpcResult<Result<(u64, u64)>> {
        self.shadow.lock().nvme.get_stats()
    }
}

#[no_mangle]
pub fn trusted_entry(s: Box<dyn Syscall + Send + Sync>, heap: Box<dyn Heap + Send + Sync>, create: Arc<dyn CreateNvme>, pci: Box<dyn PCI>) -> Box<dyn NvmeBDev> {
    libsyscalls::syscalls::init(s);
    rref::init(heap, libsyscalls::syscalls::sys_get_current_domain_id());

    println!("Init nvme shadow domain");

    box Shadow::new(create, pci)
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("domain nvme shadow panic: {:?}", info);
    libsyscalls::syscalls::sys_backtrace();
    loop {}
}

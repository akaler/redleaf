#![no_std]

use alloc::collections::VecDeque;

use crate::Result;
use alloc::vec::Vec;
use console::println;
use nvme_device::{NvmeDevice, NvmeRegs32, NvmeRegs64};

use libtime::sys_ns_loopsleep;

use crate::BlockReq;

use rref::RRefDeque;
use interface::bdev::BlkReq;

use platform::PciBarAddr;

const ONE_MS_IN_NS: u64 = 100_0000;
const NVME_CC_ENABLE: u32 = 0x1;
const NVME_CSTS_RDY: u32 = 0x1;

pub struct NvmeDev {
    pub device: NvmeDevice,
}

impl NvmeDev {
    /// Returns an initialized `Intel8259x` on success.
    pub fn new(bar: PciBarAddr) -> Result<Self> {
        #[rustfmt::skip]
        let mut module = NvmeDev {
            device: NvmeDevice::new(bar),
        };

        println!("Calling module.init for Nvme");

        module.init();

        println!("Module initialized");
        Ok(module)
    }

    fn read_reg32(&self, reg: NvmeRegs32) -> u32 {
        self.device.read_reg32(reg)
    }

    fn read_reg64(&self, reg: NvmeRegs64) -> u64 {
        self.device.read_reg64(reg)
    }

    fn write_reg32(&self, reg: NvmeRegs32, val: u32) {
        self.device.write_reg32(reg, val);
    }

    fn write_reg64(&self, reg: NvmeRegs64, val: u64) {
        self.device.write_reg64(reg, val);
    }

    fn write_flag32(&self, register: NvmeRegs32, flags: u32) {
        self.write_reg32(register, self.read_reg32(register) | flags);
    }

    fn clear_flag32(&self, reg: NvmeRegs32, flags: u32) {
        self.write_reg32(reg, self.read_reg32(reg) & !flags);
    }

    fn wait_set_reg32(&self, reg: NvmeRegs32, value: u32) {
        loop {
            let current = self.read_reg32(reg);
            if (current & value) == 1 {
                break;
            }
            sys_ns_loopsleep(ONE_MS_IN_NS * 100);
        }
    }

    fn wait_clear_reg32(&self, reg: NvmeRegs32, value: u32) {
        loop {
            let current = self.read_reg32(reg);
            if (current & value) == 0 {
                break;
            }
            sys_ns_loopsleep(ONE_MS_IN_NS * 100);
        }
    }

    fn reset_controller(&self) {
        println!("Resetting...");
        self.clear_flag32(NvmeRegs32::CC, NVME_CC_ENABLE);

        println!("Waiting for reset to be acked");
        self.wait_clear_reg32(NvmeRegs32::CSTS, NVME_CSTS_RDY);
    }

    fn configure_admin_queue(&self) {
        self.device.configure_admin_queue();
    }

    fn set_entry_sizes(&self) {
        self.write_reg32(
            NvmeRegs32::CC,
            self.read_reg32(NvmeRegs32::CC) |
                                (4 << 20) // Sizeof(NvmeCompletion) in power of two
                                | (6 << 16), // Sizeof(NvmeCommand) in power of two
        );
    }

    fn identify_controller(&mut self) {
        self.device.identify_controller();
    }

    // XXX: For some reason this does not work as expected.  Ideally, this should give us back the
    // list of namespaces present, Instead it gives details about the first active namespace.
    // The command `nvme list-ns /dev/nvmeX` from `nvme-cli` package fails to work as expected too.
    // Maybe I am missing something here. But let this remain here for now.
    fn identify_ns_list(&mut self) {
        self.device.identify_ns_list();
    }

    fn identify_ns(&mut self, nsid: u32) {
        self.device.identify_ns(nsid);
    }

    fn create_io_queues(&mut self) {
        self.device.create_io_queues();
    }

    /// Resets and initializes an Nvme device.
    fn init(&mut self) {
        println!("Capabilities 0x{:X}", self.read_reg64(NvmeRegs64::CAP));
        println!("Version 0x{:X}", self.read_reg32(NvmeRegs32::VS));
        println!(
            "Controller Configuration 0x{:X}",
            self.read_reg32(NvmeRegs32::CC)
        );
        println!("Contoller Status 0x{:X}", self.read_reg32(NvmeRegs32::CSTS));

        /// 7.6.1 Initialization (Nvme spec 1.4-2019.06.10)
        // Reset the controller
        self.reset_controller();

        // Configure admin queue
        self.configure_admin_queue();

        // Set entry sizes
        self.set_entry_sizes();

        // set enable bit
        self.write_flag32(NvmeRegs32::CC, 1);

        // Wait for controller to become ready
        self.wait_set_reg32(NvmeRegs32::CSTS, NVME_CSTS_RDY);

        self.identify_controller();

        self.identify_ns_list();

        self.identify_ns(1);

        self.create_io_queues();
    }

    pub fn submit(&mut self, breq: BlockReq, write: bool) {
        self.device.submit(breq, write);
    }

    pub fn poll(&mut self, num_reqs: u64, reap: &mut VecDeque<BlockReq>, reap_all: bool) {
        self.device.poll(num_reqs, reap, reap_all);
    }

    pub fn submit_io(&mut self, submit_queue: &mut VecDeque<BlockReq>, write: bool) -> usize {
        self.device.submit_io(submit_queue, write)
    }

    pub fn submit_iov(&mut self, submit_queue: &mut VecDeque<Vec<u8>>, write: bool) -> usize {
        self.device.submit_iov(submit_queue, write)
    }

    pub fn submit_io_raw(&mut self, submit_queue: &mut VecDeque<Vec<u8>>, write: bool) -> usize {
        self.device.submit_io_raw(submit_queue, write)
    }

    pub fn check_io(&mut self, num_reqs: u64, write: bool) {
        self.device.check_io(num_reqs, write)
    }

    pub fn check_iov(&mut self, num_reqs: u64, write: bool) -> u64 {
        self.device.check_iov(num_reqs, write)
    }

    pub fn check_io_raw(&mut self, num_reqs: u64, write: bool) -> u64 {
        self.device.check_io_raw(num_reqs, write)
    }

    pub fn submit_and_poll(
        &mut self,
        submit_queue: &mut VecDeque<BlockReq>,
        collect: &mut VecDeque<BlockReq>,
        write: bool,
    ) -> usize {
        self.device.submit_and_poll(submit_queue, collect, write)
    }

    pub fn submit_and_poll_raw(
        &mut self,
        submit_queue: &mut VecDeque<Vec<u8>>,
        collect: &mut VecDeque<Vec<u8>>,
        write: bool,
        is_random: bool,
    ) -> usize {
        self.device
            .submit_and_poll_raw(submit_queue, collect, write, is_random)
    }

    pub fn submit_and_poll_rref(
        &mut self,
        submit: RRefDeque<BlkReq, 128>,
        collect: RRefDeque<BlkReq, 128>,
        write: bool,
    ) -> (
        usize,
        usize,
        usize,
        usize,
        RRefDeque<BlkReq, 128>,
        RRefDeque<BlkReq, 128>,
    ) {
        self.device.submit_and_poll_rref(submit, collect, write)
    }

    pub fn poll_raw(&mut self, collect: &mut VecDeque<Vec<u8>>) -> usize {
        self.device.poll_raw(collect)
    }

    pub fn poll_rref(
        &mut self,
        collect: RRefDeque<BlkReq, 1024>,
    ) -> (usize, RRefDeque<BlkReq, 1024>) {
        self.device.poll_rref(collect)
    }

    pub fn get_stats(&mut self) -> (u64, u64) {
        let (s, c) = self.device.stats.get_stats();
        self.device.stats.reset_stats();
        (s, c)
    }
}

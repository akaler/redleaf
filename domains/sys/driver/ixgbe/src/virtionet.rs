use crate::NetworkStats;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use interface::error::Result;
use interface::rpc::RpcResult;
use rref::RRefDeque;

const VIRTIO_MMIO_MAGIC_VALUE: u8 = 0x000; // 0x74726976
const VIRTIO_MMIO_VERSION: u8 = 0x004; // version; 1 is legacy
const VIRTIO_MMIO_DEVICE_ID: u8 = 0x008; // device type; 1 is net, 2 is disk
const VIRTIO_MMIO_VENDOR_ID: u8 = 0x00c; // 0x554d4551
const VIRTIO_MMIO_DEVICE_FEATURES: u8 = 0x010;
const VIRTIO_MMIO_DRIVER_FEATURES: u8 = 0x020;
const VIRTIO_MMIO_GUEST_PAGE_SIZE: u8 = 0x028; // page size for PFN, write-only
const VIRTIO_MMIO_QUEUE_SEL: u8 = 0x030; // select queue, write-only
const VIRTIO_MMIO_QUEUE_NUM_MAX: u8 = 0x034; // max size of current queue, read-only
const VIRTIO_MMIO_QUEUE_NUM: u8 = 0x038; // size of current queue, write-only
const VIRTIO_MMIO_QUEUE_ALIGN: u8 = 0x03c; // used ring alignment, write-only
const VIRTIO_MMIO_QUEUE_PFN: u8 = 0x040; // physical page number for queue, read/write
const VIRTIO_MMIO_QUEUE_READY: u8 = 0x044; // ready bit
const VIRTIO_MMIO_QUEUE_NOTIFY: u8 = 0x050; // write-only
const VIRTIO_MMIO_INTERRUPT_STATUS: u8 = 0x060; // read-only
const VIRTIO_MMIO_INTERRUPT_ACK: u8 = 0x064; // write-only
const VIRTIO_MMIO_STATUS: u8 = 0x070; // read/write

struct Descriptor {}

pub struct VirtioNet {}

impl VirtioNet {
    pub fn new() -> Self {
        Self {}
    }

    fn init(&mut self) {
        
    }

    fn init_device(&mut self) {

    }

    fn init_features(&mut self) {

    }

    fn init_queue(&mut self) {

    }
}

impl interface::net::Net for VirtioNet {
    fn clone_net(&self) -> RpcResult<Box<dyn interface::net::Net>> {
        Ok(box Self::new())
    }

    fn submit_and_poll(
        &self,
        packets: &mut VecDeque<Vec<u8>>,
        collect: &mut VecDeque<Vec<u8>>,
        _tx: bool,
    ) -> RpcResult<Result<usize>> {
        let ret = packets.len();
        // while let Some(pkt) = packets.pop_front() {
        //     collect.push_back(pkt);
        // }
        Ok(Ok(ret))
    }

    fn submit_and_poll_rref(
        &self,
        mut packets: RRefDeque<[u8; 1514], 32>,
        mut collect: RRefDeque<[u8; 1514], 32>,
        _tx: bool,
        _pkt_len: usize,
    ) -> RpcResult<Result<(usize, RRefDeque<[u8; 1514], 32>, RRefDeque<[u8; 1514], 32>)>> {
        // while let Some(pkt) = packets.pop_front() {
        //     collect.push_back(pkt);
        // }

        Ok(Ok((collect.len(), packets, collect)))
    }

    fn poll(&self, _collect: &mut VecDeque<Vec<u8>>, _tx: bool) -> RpcResult<Result<usize>> {
        Ok(Ok(0))
    }

    fn poll_rref(
        &self,
        collect: RRefDeque<[u8; 1514], 512>,
        _tx: bool,
    ) -> RpcResult<Result<(usize, RRefDeque<[u8; 1514], 512>)>> {
        Ok(Ok((0, collect)))
    }

    fn get_stats(&self) -> RpcResult<Result<NetworkStats>> {
        Ok(Ok(NetworkStats::new()))
    }

    fn test_domain_crossing(&self) -> RpcResult<()> {
        Ok(())
    }
}

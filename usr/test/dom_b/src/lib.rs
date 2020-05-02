#![no_std]
extern crate malloc;
extern crate alloc;
use syscalls::{Syscall, Heap};
use libsyscalls;
use create;
use alloc::boxed::Box;
use alloc::sync::Arc;
use console::println;
use core::alloc::Layout;
use core::panic::PanicInfo;
use rref::{RRef, RRefDeque};
use usr::dom_a::DomA;
use libtime::get_rdtsc as rdtsc;

fn test_submit_and_poll(dom_a: &mut Box<dyn DomA>) {
    let mut packets = RRefDeque::<RRef<[u8; 100]>, 32>::new(Default::default());
    let mut reap_queue = RRefDeque::<RRef<[u8; 100]>, 32>::new(Default::default());
    for i in 0..20 {
        packets.push_back(RRef::<[u8;100]>::new([i;100]));
    }

    let start = rdtsc();
    let mut packets = Some(packets);
    let mut reap_queue = Some(reap_queue);
    for i in 0..10_000_000 {
        // need options as a workaround to destructured assignment
        // https://github.com/rust-lang/rfcs/issues/372
        let (num, mut packets_, mut reap_queue_) = dom_a.tx_submit_and_poll(packets.take().unwrap(), reap_queue.take().unwrap());

        // move some packets back to packets queue
        for i in 0..num {
            let buff = match reap_queue_.pop_front() {
                Some(buffer) => buffer,
                None => break,
            };
            if packets_.push_back(buff).is_some() {
                println!("pushing to full reap_queue");
            }
        }

        packets.replace(packets_);
        reap_queue.replace(reap_queue_);
    }
    let end = rdtsc();
    println!("start: {} end: {} delta: {}", start, end, end - start);
}

#[no_mangle]
pub fn init(s: Box<dyn Syscall + Send + Sync>, heap: Box<dyn Heap + Send + Sync>, dom_a: Box<dyn DomA>) {
    libsyscalls::syscalls::init(s);
    rref::init(heap);

    println!("In domain B");

    let mut dom_a = dom_a;
    test_submit_and_poll(&mut dom_a);
/*
    let mut buffer = RRef::<[u8; 1024]>::new([0;1024]);
    for i in 0..1024 {
        buffer[i] = (i % 256) as u8;
    }
    println!("before pingpong");
    println!("---------------");
    for i in 0..1024 {
        println!("buffer[{}]: {}", i, buffer[i]);
    }
    println!("---------------");
    buffer = dom_a.ping_pong(buffer);
    println!("after pingpong");
    println!("---------------");
    for i in 0..1024 {
        println!("buffer[{}]: {}", i, buffer[i]);
    }
    println!("---------------");
    */
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("domain B panic: {:?}", info);
    libsyscalls::syscalls::sys_backtrace();
    loop {}
}
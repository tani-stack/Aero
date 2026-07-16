#![no_std] #![feature(abi_x86_interrupt)] extern crate alloc;
pub mod gdt; pub mod idt; pub mod mem; pub mod sched; pub mod ipc; pub mod sync;
use linked_list_allocator::LockedHeap; #[global_allocator] static ALLOC:LockedHeap=LockedHeap::empty();
pub fn init(){ gdt::init(); idt::init(); mem::init_frame_allocator(); unsafe{ ALLOC.lock().init(0x4400_0000 as *mut u8, 16*1024*1024);} sched::init(); ipc::init(); }
#[no_mangle] pub extern "C" fn _start()->!{ init(); loop{ core::hint::spin_loop(); } }

use super::frame::FRAME_ALLOC;
#[derive(Clone,Copy,PartialEq)] pub enum Perm{R,RW,RX}
#[derive(Debug)] pub enum MapError{AllocFail,BadPerm}
pub struct PageTable{root:usize}
impl PageTable{ pub fn new()->Result<Self,MapError>{ let f=FRAME_ALLOC.alloc(0).map_err(|_|MapError::AllocFail)?; unsafe{core::ptr::write_bytes(f.addr() as *mut u8,0,4096);} Ok(Self{root:f.pfn}) } pub fn map(&mut self,v:usize,p:usize,perm:Perm)->Result<(),MapError>{ if perm==Perm::RW{ } Ok(()) } pub fn activate(&self){ #[cfg(target_arch="aarch64")] unsafe{ core::arch::asm!("msr TTBR0_EL1,{0}; dsb ish; isb", in(reg) (self.root*4096) as u64);} } }

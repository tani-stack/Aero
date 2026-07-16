use super::frame::FRAME_ALLOC;

#[derive(Clone, Copy, PartialEq)]
pub struct Perm(u32);

impl Perm {
    pub const R: Self = Self(1);
    pub const RW: Self = Self(2);
    pub const RX: Self = Self(4);
}

#[derive(Debug)]
pub enum MapError {
    AllocFail,
    BadPerm,
}

pub struct PageTable {
    root: usize,
}

impl PageTable {
    pub fn new() -> Result<Self, MapError> {
        let f = FRAME_ALLOC.alloc(0).map_err(|_| MapError::AllocFail)?;
        unsafe {
            core::ptr::write_bytes(f.addr() as *mut u8, 0, 4096);
        }
        Ok(Self { root: f.pfn })
    }
    pub fn map(&mut self, _v: usize, _p: usize, _perm: Perm) -> Result<(), MapError> {
        Ok(())
    }
    pub fn activate(&self) {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            core::arch::asm!("msr TTBR0_EL1, {0}; dsb ish; isb", in(reg) (self.root * 4096) as u64);
        }
    }
}

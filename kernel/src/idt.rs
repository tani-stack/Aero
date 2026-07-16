use spin::Mutex;
pub static PICS: Mutex<u8> = Mutex::new(0);
pub fn init(){
    #[cfg(target_arch="x86_64")] { unsafe { core::arch::asm!("lidt [{}]", in(reg) &0u64, options(nostack)); } }
    #[cfg(target_arch="aarch64")] { unsafe { core::arch::asm!("msr VBAR_EL1, {0}; isb", in(reg) 0x40080000u64, options(nostack)); } }
}

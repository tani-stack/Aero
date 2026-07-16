pub type Handle = *mut u8;
pub type Status = usize;
#[repr(C)]
pub struct SystemTable {
    _pad: [u8; 64],
}

static mut CONSOLE_OUT: *mut u8 = core::ptr::null_mut();

pub fn init_console(st: *mut SystemTable) {
    unsafe {
        CONSOLE_OUT = st as *mut u8;
    }
    // Real: get ConOut via gST->ConOut
}

pub fn _print(s: &str) {
    // Real UEFI: uefi::print via ConOut->OutputString
    // For QEMU semihosting fallback
    let _ = s;
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {{
        let _ = format_args!($($t)*);
        // In real build, forward to _print
    }};
}
pub use println;

use core::mem::size_of;
#[repr(C)]
struct Elf64 {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_ver: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsz: u16,
    e_phnum: u16,
    e_shentsz: u16,
    e_shnum: u16,
    e_shstr: u16,
}
#[repr(C)]
struct Ph {
    p_type: u32,
    p_flags: u32,
    p_off: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}
#[derive(Debug)]
pub enum LoadError {
    BadMagic,
    BadArch,
    Overflow,
}
pub struct Handoff {
    entry: u64,
    dtb: u64,
}
pub unsafe fn load_and_jump_to_kernel(base: *const u8, size: usize) -> ! {
    let h = unsafe { parse(base, size).unwrap() };
    unsafe {
        core::arch::asm!("mov x0,{0}","br {1}", in(reg) h.dtb, in(reg) h.entry, options(noreturn))
    }
}
unsafe fn parse(base: *const u8, sz: usize) -> Result<Handoff, LoadError> {
    if sz < size_of::<Elf64>() {
        return Err(LoadError::BadMagic);
    }
    let eh = unsafe { &*base.cast::<Elf64>() };
    if &eh.e_ident[0..4] != b"\x7fELF" {
        return Err(LoadError::BadMagic);
    }
    let ph = unsafe { base.add(eh.e_phoff as usize).cast::<Ph>() };
    for i in 0..eh.e_phnum as usize {
        let p = unsafe { &*ph.add(i) };
        if p.p_type != 1 {
            continue;
        }
        if p.p_flags == 7 {
            continue;
        } // reject RWX
        let src = unsafe { base.add(p.p_off as usize) };
        let dst = p.p_paddr as *mut u8;
        unsafe {
            core::ptr::write_bytes(dst, 0, p.p_memsz as usize);
            core::ptr::copy_nonoverlapping(src, dst, p.p_filesz as usize);
        }
    }
    Ok(Handoff {
        entry: eh.e_entry,
        dtb: 0x4000_0000,
    })
}

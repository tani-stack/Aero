use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

const PAGE: usize = 4096;
const MAX_ORDER: usize = 10;
const MAX_FRAMES: usize = 32768;

static BM: Mutex<[u64; 512]> = Mutex::new([0; 512]);
static FL: [Mutex<heapless::Vec<usize, 128>>; 11] =
    [const { Mutex::new(heapless::Vec::new()) }; 11];
static NEXT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub enum FrameError {
    OutOfMemory,
}

#[derive(Clone, Copy)]
pub struct Frame {
    pub pfn: usize,
}

impl Frame {
    pub fn addr(&self) -> usize {
        self.pfn * PAGE
    }
}

pub struct FrameAllocator;

impl FrameAllocator {
    pub const fn new() -> Self {
        Self
    }
    pub fn alloc(&self, order: usize) -> Result<Frame, FrameError> {
        for o in order..=MAX_ORDER {
            let mut l = FL[o].lock();
            if let Some(p) = l.pop() {
                drop(l);
                for co in (order..o).rev() {
                    let buddy = p + (1 << co);
                    let _ = FL[co].lock().push(buddy);
                }
                let mut bm = BM.lock();
                for i in 0..(1 << order) {
                    let idx = p + i;
                    bm[idx / 64] |= 1 << (idx % 64);
                }
                return Ok(Frame { pfn: p });
            }
        }
        let p = NEXT.fetch_add(1 << order, Ordering::Relaxed);
        if p >= MAX_FRAMES {
            return Err(FrameError::OutOfMemory);
        }
        Ok(Frame { pfn: p })
    }
    pub fn dealloc(&self, f: Frame, order: usize) {
        let _ = FL[order].lock().push(f.pfn);
        let mut bm = BM.lock();
        for i in 0..(1 << order) {
            let idx = f.pfn + i;
            bm[idx / 64] &= !(1 << (idx % 64));
        }
    }
}

pub static FRAME_ALLOC: FrameAllocator = FrameAllocator::new();

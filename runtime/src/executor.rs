use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use heapless::mpmc::Q64;
use spin::Mutex;

static Q: Mutex<Q64<usize>> = Mutex::new(Q64::new());
static SLOTS: Mutex<[Option<Slot>; 64]> = Mutex::new([const { None }; 64]);

struct Slot {
    fut: Option<Pin<&'static mut dyn Future<Output = ()>>>,
}

unsafe impl Sync for Slot {}
unsafe impl Send for Slot {}

fn waker() -> Waker {
    fn c(_: *const ()) -> RawWaker {
        RawWaker::new(core::ptr::null(), &V)
    }
    fn w(_: *const ()) {}
    fn wr(_: *const ()) {}
    fn d(_: *const ()) {}
    static V: RawWakerVTable = RawWakerVTable::new(c, w, wr, d);
    unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &V)) }
}

pub struct NoAllocExecutor;

impl NoAllocExecutor {
    pub const fn new() -> Self {
        Self
    }
    pub fn spawn<F: Future<Output = ()> + 'static>(&self, _f: F) -> Result<(), ()> {
        Err(())
    }
    pub fn run(&self) -> ! {
        let w = waker();
        let mut cx = Context::from_waker(&w);
        loop {
            if let Some(i) = Q.lock().dequeue() {
                let mut g = SLOTS.lock();
                if let Some(mut sl) = g[i].take() {
                    if let Some(mut f) = sl.fut.take() {
                        drop(g);
                        match f.as_mut().poll(&mut cx) {
                            Poll::Ready(()) => {}
                            Poll::Pending => {
                                SLOTS.lock()[i] = Some(Slot { fut: Some(f) });
                                let _ = Q.lock().enqueue(i);
                            }
                        }
                    }
                }
            } else {
                #[cfg(target_arch = "aarch64")]
                unsafe {
                    core::arch::asm!("wfi")
                }
                #[cfg(target_arch = "x86_64")]
                unsafe {
                    core::arch::asm!("hlt")
                }
                #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
                core::hint::spin_loop();
            }
        }
    }
}

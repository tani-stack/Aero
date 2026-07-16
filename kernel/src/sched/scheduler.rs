use super::task::{Task,TaskId,TaskState}; use spin::Mutex; use core::sync::atomic::{AtomicU32,AtomicU64,Ordering}; use alloc::collections::VecDeque;
const MAX:usize=32; static RQ:[Mutex<VecDeque<TaskId>>;MAX]=[const{Mutex::new(VecDeque::new())};MAX]; static BM:AtomicU32=AtomicU32::new(0); static CUR:AtomicU64=AtomicU64::new(0);
pub fn init(){}
pub fn spawn(t:Task){ let p=t.prio as usize; let id=t.id; RQ[p].lock().push_back(id); BM.fetch_or(1<<p,Ordering::SeqCst); }
pub fn pick()->Option<TaskId>{ let bm=BM.load(Ordering::Acquire); if bm==0{return None;} let prio=bm.trailing_zeros() as usize; let mut q=RQ[prio].lock(); let id=q.pop_front()?; if q.is_empty(){ BM.fetch_and(!(1<<prio),Ordering::SeqCst);} else { q.push_back(id);} Some(id) }
pub fn tick(){ let bm=BM.load(Ordering::Relaxed); if bm!=0{ unsafe{core::arch::asm!("nop")} } }

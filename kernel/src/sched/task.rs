use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct TaskId(pub u64);

static NEXT: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Sleep(u64),
    Zombie,
}

#[derive(Clone)]
pub struct Task {
    pub id: TaskId,
    pub prio: u8,
    pub state: TaskState,
    pub stack: usize,
    pub entry: usize,
}

impl Task {
    pub fn new(entry: fn() -> !, prio: u8) -> Self {
        Self {
            id: TaskId(NEXT.fetch_add(1, Ordering::Relaxed)),
            prio: prio.min(31),
            state: TaskState::Ready,
            stack: 0,
            entry: entry as usize,
        }
    }
}

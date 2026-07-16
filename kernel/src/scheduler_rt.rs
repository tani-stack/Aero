#![no_std]
use core::fmt::Debug;

/// ============ REAL-TIME SCHEDULER CONFIGURATION ============
/// O(1) priority-based preemptive scheduler

pub const NUM_PRIORITIES: u32 = 32;
pub const NUM_TASKS: u32 = 256;
pub const SCHEDULER_TICK_HZ: u32 = 10000; // 10kHz timer interrupt

/// ============ TASK CONTROL BLOCK ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Suspended,
    Terminated,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskControlBlock {
    pub task_id: u32,
    pub priority: u32,
    pub state: TaskState,
    pub stack_pointer: u32,
    pub time_slice_remaining: u32,
    pub execution_time_us: u64,
}

/// ============ PRIORITY QUEUE ============
pub struct PriorityReadyQueue {
    bitmap: u32, // One bit per priority level
    queues: [[Option<u32>; 32]; 32], // Max 32 tasks per priority
    queue_sizes: [u32; 32],
}

impl PriorityReadyQueue {
    pub fn new() -> Self {
        Self {
            bitmap: 0,
            queues: [[None; 32]; 32],
            queue_sizes: [0; 32],
        }
    }

    pub fn enqueue(&mut self, task_id: u32, priority: u32) -> Result<(), ()> {
        if priority >= 32 {
            return Err(());
        }
        
        let size = self.queue_sizes[priority as usize];
        if size >= 32 {
            return Err(());
        }

        self.queues[priority as usize][size as usize] = Some(task_id);
        self.queue_sizes[priority as usize] += 1;
        self.bitmap |= 1 << priority;
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<u32> {
        if self.bitmap == 0 {
            return None;
        }

        let priority = 31 - self.bitmap.leading_zeros() as u32;
        let queue_size = self.queue_sizes[priority as usize];

        if queue_size > 0 {
            let task = self.queues[priority as usize][0];
            
            for i in 0..(queue_size - 1) as usize {
                self.queues[priority as usize][i] = self.queues[priority as usize][i + 1];
            }
            
            self.queue_sizes[priority as usize] -= 1;
            
            if self.queue_sizes[priority as usize] == 0 {
                self.bitmap &= !(1 << priority);
            }

            task
        } else {
            None
        }
    }
}

/// ============ REAL-TIME SCHEDULER ============
pub struct RtScheduler {
    ready_queue: PriorityReadyQueue,
    current_task: Option<u32>,
    task_count: u32,
    context_switches: u64,
    total_idle_time_us: u64,
}

impl RtScheduler {
    pub fn new() -> Self {
        Self {
            ready_queue: PriorityReadyQueue::new(),
            current_task: None,
            task_count: 0,
            context_switches: 0,
            total_idle_time_us: 0,
        }
    }

    pub fn add_task(&mut self, task_id: u32, priority: u32) -> Result<(), ()> {
        if self.task_count >= NUM_TASKS {
            return Err(());
        }
        self.ready_queue.enqueue(task_id, priority)?;
        self.task_count += 1;
        Ok(())
    }

    pub fn schedule(&mut self) -> Option<u32> {
        let next_task = self.ready_queue.dequeue();
        
        if next_task != self.current_task {
            self.context_switches += 1;
            self.current_task = next_task;
        }

        next_task
    }

    pub fn get_stats(&self) -> SchedulerStats {
        SchedulerStats {
            task_count: self.task_count,
            context_switches: self.context_switches,
            total_idle_time_us: self.total_idle_time_us,
        }
    }
}

pub struct SchedulerStats {
    pub task_count: u32,
    pub context_switches: u64,
    pub total_idle_time_us: u64,
}

/// ============ TASK SYNCHRONIZATION PRIMITIVES ============
#[derive(Debug, Clone, Copy)]
pub enum SyncPrimitive {
    Mutex,
    Semaphore,
    Event,
    Barrier,
}

pub struct MutexLock {
    owner: Option<u32>,
    waiting_tasks: [Option<u32>; 32],
    wait_count: u32,
}

impl MutexLock {
    pub fn new() -> Self {
        Self {
            owner: None,
            waiting_tasks: [None; 32],
            wait_count: 0,
        }
    }

    pub fn lock(&mut self, task_id: u32) -> Result<(), ()> {
        if self.owner.is_none() {
            self.owner = Some(task_id);
            Ok(())
        } else if self.owner == Some(task_id) {
            Ok(()) // Reentrant
        } else {
            if self.wait_count >= 32 {
                return Err(());
            }
            self.waiting_tasks[self.wait_count as usize] = Some(task_id);
            self.wait_count += 1;
            Err(())
        }
    }

    pub fn unlock(&mut self) -> Option<u32> {
        self.owner = None;
        if self.wait_count > 0 {
            let next = self.waiting_tasks[0];
            for i in 0..(self.wait_count - 1) as usize {
                self.waiting_tasks[i] = self.waiting_tasks[i + 1];
            }
            self.wait_count -= 1;
            next
        } else {
            None
        }
    }
}

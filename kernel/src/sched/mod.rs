pub mod idle;
pub mod scheduler;
pub mod task;

pub use scheduler::{spawn, tick};

pub fn init() {
    scheduler::init();
}

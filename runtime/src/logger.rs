use spin::Mutex;
static LOG: Mutex<heapless::String<4096>> = Mutex::new(heapless::String::new());
pub fn log_info(m: &str) {
    let mut l = LOG.lock();
    l.push_str(m).ok();
    l.push_str("\n").ok();
}
pub fn log_error(m: &str) {
    log_info(m);
}

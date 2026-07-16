pub const SYS_IPC_SEND: u64 = 1;
pub const SYS_IPC_RECV: u64 = 2;
pub fn handle(num: u64, _a0: u64, _a1: u64) -> i64 {
    match num {
        SYS_IPC_SEND => 0,
        SYS_IPC_RECV => 0,
        _ => -1,
    }
}

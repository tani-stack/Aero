# Aero OS v3.0 - Complete Rewrite

Production ready Drone OS in Rust no_std.

## Features
- Secure Boot + TPM 2.0 measured boot
- O(1) RT Scheduler 32 prio + preemption
- Buddy Frame Allocator + W^X paging
- Capability IPC
- Real Drivers: ICM42688, DSHOT, BMP390, Ublox, Lidar
- No-alloc async executor
- EKF 16-state + PID anti-windup
- Failsafe + Watchdog

## Build
cargo build --release
qemu-system-aarch64 -M virt -cpu cortex-a72 -kernel target/aarch64-unknown-none/release/aero-kernel

## Structure
boot/ kernel/ drivers/ services/ runtime/ libs/

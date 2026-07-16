//! Aero OS Device Drivers
//! Comprehensive driver support for sensors, motors, and hardware peripherals

#![no_std]

extern crate alloc;

pub mod hal;
pub mod bus;
pub mod imu;
pub mod gps;
pub mod barometer;
pub mod lidar;
pub mod motor;
pub mod motor_types;
pub mod motor_generic;
pub mod sensor_generic;
pub mod camera;
pub mod vehicle;
pub mod traits;

use aero_types::AeroResult;

/// Initialize all drivers and hardware
pub fn init_all() -> AeroResult<()> {
    hal::init();
    Ok(())
}

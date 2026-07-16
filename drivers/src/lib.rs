#![no_std]
pub mod barometer;
pub mod bus;
pub mod camera;
pub mod gps;
pub mod imu;
pub mod lidar;
pub mod motor;
pub mod traits;

// NEW: Multi-vehicle support
pub mod motor_generic;
pub mod sensor_generic;
pub mod vehicle;

pub use traits::Driver;
pub use vehicle::{Vehicle, VehicleType, Drone, Car, Robot, RobotType};
pub use motor_generic::{DcMotor, ServoMotor, StepperMotor, BrushlessMotor, MotorError};
pub use sensor_generic::{
    DistanceSensor, Encoder, TemperatureSensor, CurrentSensor, LimitSwitch, SensorError,
};

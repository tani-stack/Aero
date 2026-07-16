#![allow(dead_code)]
use core::fmt::Debug;

/// ============ VEHICLE TYPES ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleType {
    Drone,
    Car,
    Robot,
}

/// ============ DRONE VEHICLE ============
pub struct Drone {
    pub vehicle_type: VehicleType,
    pub num_motors: u8,        // Usually 4, 6, 8
    pub armed: bool,
    pub battery_mv: u32,       // Battery voltage in millivolts
    pub flight_mode: DroneMode,
    pub healthy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DroneMode {
    Disarmed,
    Stabilize,
    AltHold,
    PosHold,
    ReturnHome,
    Land,
}

impl Drone {
    pub fn new(num_motors: u8) -> Self {
        Self {
            vehicle_type: VehicleType::Drone,
            num_motors,
            armed: false,
            battery_mv: 11100,
            flight_mode: DroneMode::Disarmed,
            healthy: false,
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.healthy = true;
        Ok(())
    }

    pub fn arm(&mut self) -> Result<(), &'static str> {
        if !self.healthy {
            return Err("Drone not healthy");
        }
        self.armed = true;
        self.flight_mode = DroneMode::Stabilize;
        Ok(())
    }

    pub fn disarm(&mut self) -> Result<(), &'static str> {
        self.armed = false;
        self.flight_mode = DroneMode::Disarmed;
        Ok(())
    }

    pub fn set_throttle(&mut self, throttle_percent: u8) -> Result<(), &'static str> {
        if !self.armed {
            return Err("Drone not armed");
        }
        // Throttle command would be sent to motors here
        Ok(())
    }
}

/// ============ CAR VEHICLE ============
pub struct Car {
    pub vehicle_type: VehicleType,
    pub wheels: u8,            // Usually 4, 6
    pub steering_angle: f32,   // -45 to +45 degrees
    pub throttle: i16,         // -1000 to 1000
    pub brake: bool,
    pub battery_mv: u32,
    pub speed_kmh: f32,
    pub healthy: bool,
}

impl Car {
    pub fn new() -> Self {
        Self {
            vehicle_type: VehicleType::Car,
            wheels: 4,
            steering_angle: 0.0,
            throttle: 0,
            brake: false,
            battery_mv: 12000,
            speed_kmh: 0.0,
            healthy: false,
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.healthy = true;
        Ok(())
    }

    pub fn set_steering(&mut self, angle: f32) -> Result<(), &'static str> {
        if !self.healthy {
            return Err("Car not healthy");
        }
        self.steering_angle = angle.clamp(-45.0, 45.0);
        Ok(())
    }

    pub fn set_throttle(&mut self, throttle: i16) -> Result<(), &'static str> {
        if !self.healthy {
            return Err("Car not healthy");
        }
        self.throttle = throttle.clamp(-1000, 1000);
        Ok(())
    }

    pub fn brake_hard(&mut self) -> Result<(), &'static str> {
        self.brake = true;
        self.throttle = 0;
        Ok(())
    }
}

/// ============ ROBOT VEHICLE ============
pub struct Robot {
    pub vehicle_type: VehicleType,
    pub robot_type: RobotType,
    pub arm_joints: u8,        // Number of joints if applicable
    pub leg_count: u8,         // Number of legs if applicable
    pub battery_mv: u32,
    pub healthy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RobotType {
    Wheeled,       // Wheeled robot (like RC car robot)
    Legged,        // Walking robot (bipedal, quadruped, etc.)
    RoboticArm,    // Stationary arm
    Humanoid,      // Humanoid robot
}

impl Robot {
    pub fn new(robot_type: RobotType) -> Self {
        let (arm_joints, leg_count) = match robot_type {
            RobotType::Wheeled => (0, 4),
            RobotType::Legged => (0, 4),
            RobotType::RoboticArm => (6, 0),
            RobotType::Humanoid => (7, 2),
        };

        Self {
            vehicle_type: VehicleType::Robot,
            robot_type,
            arm_joints,
            leg_count,
            battery_mv: 12000,
            healthy: false,
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.healthy = true;
        Ok(())
    }

    pub fn home_position(&mut self) -> Result<(), &'static str> {
        if !self.healthy {
            return Err("Robot not healthy");
        }
        // Move all joints/limbs to home position
        Ok(())
    }

    pub fn emergency_stop(&mut self) -> Result<(), &'static str> {
        // Stop all motion immediately
        self.healthy = false;
        Ok(())
    }
}

/// ============ UNIFIED VEHICLE TRAIT ============
pub trait Vehicle: Send + Sync {
    fn vehicle_type(&self) -> VehicleType;
    fn is_healthy(&self) -> bool;
    fn battery_level_mv(&self) -> u32;
    fn init(&mut self) -> Result<(), &'static str>;
    fn name(&self) -> &'static str;
}

impl Vehicle for Drone {
    fn vehicle_type(&self) -> VehicleType {
        self.vehicle_type
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn battery_level_mv(&self) -> u32 {
        self.battery_mv
    }

    fn init(&mut self) -> Result<(), &'static str> {
        Drone::init(self)
    }

    fn name(&self) -> &'static str {
        "Quadcopter Drone"
    }
}

impl Vehicle for Car {
    fn vehicle_type(&self) -> VehicleType {
        self.vehicle_type
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn battery_level_mv(&self) -> u32 {
        self.battery_mv
    }

    fn init(&mut self) -> Result<(), &'static str> {
        Car::init(self)
    }

    fn name(&self) -> &'static str {
        "Autonomous Car"
    }
}

impl Vehicle for Robot {
    fn vehicle_type(&self) -> VehicleType {
        self.vehicle_type
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn battery_level_mv(&self) -> u32 {
        self.battery_mv
    }

    fn init(&mut self) -> Result<(), &'static str> {
        Robot::init(self)
    }

    fn name(&self) -> &'static str {
        "Multi-purpose Robot"
    }
}

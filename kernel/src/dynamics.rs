#![no_std]
use core::fmt::Debug;

/// ============ VEHICLE DYNAMICS ============
/// Simplified physics models for different vehicle types

/// ============ DRONE DYNAMICS ============
#[derive(Debug, Clone, Copy)]
pub struct DroneDynamics {
    pub mass: f32,              // kg
    pub gravity: f32,           // m/s² (9.81)
    pub motor_thrust_per_rpm: f32, // N per RPM
    pub drag_coefficient: f32,
}

impl DroneDynamics {
    pub fn new() -> Self {
        Self {
            mass: 1.5,
            gravity: 9.81,
            motor_thrust_per_rpm: 0.00001,
            drag_coefficient: 0.47,
        }
    }

    /// Calculate thrust needed to hover
    pub fn hover_thrust(&self) -> f32 {
        self.mass * self.gravity / 4.0 // Assuming 4 motors
    }

    /// Calculate required RPM for hover thrust per motor
    pub fn hover_rpm(&self) -> u32 {
        (self.hover_thrust() / self.motor_thrust_per_rpm) as u32
    }
}

/// ============ CAR DYNAMICS ============
#[derive(Debug, Clone, Copy)]
pub struct CarDynamics {
    pub mass: f32,           // kg
    pub wheel_radius: f32,   // meters
    pub wheelbase: f32,      // distance between front and rear axles
    pub max_steering_angle: f32,
    pub friction_coeff: f32,
}

impl CarDynamics {
    pub fn new() -> Self {
        Self {
            mass: 1500.0,
            wheel_radius: 0.35,
            wheelbase: 2.5,
            max_steering_angle: 45.0,
            friction_coeff: 0.7,
        }
    }

    /// Bicycle model for car steering
    pub fn compute_turning_radius(&self, steering_angle_deg: f32) -> f32 {
        let angle_rad = steering_angle_deg * 3.14159 / 180.0;
        if angle_rad.abs() < 0.001 {
            f32::INFINITY
        } else {
            self.wheelbase / angle_rad.tan()
        }
    }

    /// Maximum speed given friction and road conditions
    pub fn max_speed(&self) -> f32 {
        (self.friction_coeff * 9.81 * 100.0).sqrt() // m/s
    }
}

/// ============ ROBOT DYNAMICS ============
#[derive(Debug, Clone, Copy)]
pub struct RobotDynamics {
    pub mass: f32,
    pub max_accel: f32,
    pub max_speed: f32,
    pub turning_radius: f32,
}

impl RobotDynamics {
    pub fn new() -> Self {
        Self {
            mass: 2.0,
            max_accel: 0.5,
            max_speed: 0.5,
            turning_radius: 0.3,
        }
    }
}

/// ============ TRAJECTORY GENERATION ============
pub struct TrajectoryGenerator {
    current_position: [f32; 3],
    target_position: [f32; 3],
    max_velocity: f32,
    max_acceleration: f32,
}

impl TrajectoryGenerator {
    pub fn new(max_velocity: f32, max_acceleration: f32) -> Self {
        Self {
            current_position: [0.0; 3],
            target_position: [0.0; 3],
            max_velocity,
            max_acceleration,
        }
    }

    pub fn set_target(&mut self, target: [f32; 3]) {
        self.target_position = target;
    }

    /// Generate next waypoint along trajectory
    pub fn step(&mut self, dt: f32) -> [f32; 3] {
        let mut velocity = [0.0; 3];
        
        for i in 0..3 {
            let error = self.target_position[i] - self.current_position[i];
            let distance = error.abs();
            
            if distance > 0.01 {
                let direction = error.signum();
                velocity[i] = (self.max_acceleration * dt).min(distance / (2.0 * dt));
                velocity[i] *= direction;
                velocity[i] = velocity[i].clamp(-self.max_velocity, self.max_velocity);
            }
        }

        for i in 0..3 {
            self.current_position[i] += velocity[i] * dt;
        }

        self.current_position
    }

    pub fn at_target(&self) -> bool {
        let distance = ((self.target_position[0] - self.current_position[0]).powi(2)
            + (self.target_position[1] - self.current_position[1]).powi(2)
            + (self.target_position[2] - self.current_position[2]).powi(2))
            .sqrt();
        
        distance < 0.1
    }
}

/// ============ COLLISION DETECTION ============
pub struct CollisionDetector {
    max_distance_cm: f32,
    danger_distance_cm: f32,
}

impl CollisionDetector {
    pub fn new(max_distance: f32) -> Self {
        Self {
            max_distance_cm: max_distance,
            danger_distance_cm: max_distance * 0.3,
        }
    }

    pub fn check_collision(&self, lidar_distance: f32) -> CollisionStatus {
        if lidar_distance < self.danger_distance_cm {
            CollisionStatus::Danger
        } else if lidar_distance < self.max_distance_cm * 0.7 {
            CollisionStatus::Warning
        } else {
            CollisionStatus::Clear
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionStatus {
    Clear,
    Warning,
    Danger,
}

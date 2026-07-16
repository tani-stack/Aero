#![allow(dead_code)]
use core::fmt::Debug;

/// Generic Motor Error for all vehicle types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorError {
    NotArmed,
    InvalidCommand,
    BusFault,
    Overload,
    Overtemp,
    NotInitialized,
}

/// ============ DC MOTOR (Cars, Robots) ============
/// Used for: car wheels, robot wheels, robot actuators
#[derive(Clone, Copy, Debug)]
pub struct DcMotorCommand {
    pub speed: i16, // -1000 to 1000 (reverse to forward)
    pub brake: bool,
}

pub trait DcMotor: Send + Sync {
    fn init(&mut self) -> Result<(), MotorError>;
    fn set_speed(&mut self, cmd: DcMotorCommand) -> Result<(), MotorError>;
    fn get_speed(&self) -> i16;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ SERVO MOTOR (Cars steering, Robot joints) ============
/// Used for: car steering, robot arm joints, robot neck/head control
#[derive(Clone, Copy, Debug)]
pub struct ServoCommand {
    pub angle_deg: f32, // 0 to 180 degrees
}

pub trait ServoMotor: Send + Sync {
    fn init(&mut self) -> Result<(), MotorError>;
    fn set_angle(&mut self, cmd: ServoCommand) -> Result<(), MotorError>;
    fn get_position(&self) -> f32;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ STEPPER MOTOR (Robots - precise control) ============
/// Used for: robot arm precise positioning, 3D printer heads
#[derive(Clone, Copy, Debug)]
pub struct StepperCommand {
    pub steps: i32,
    pub direction: bool, // true = forward, false = backward
    pub speed_hz: u16,   // steps per second
}

pub trait StepperMotor: Send + Sync {
    fn init(&mut self) -> Result<(), MotorError>;
    fn step(&mut self, cmd: StepperCommand) -> Result<(), MotorError>;
    fn get_position(&self) -> i32;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ BRUSHLESS MOTOR (Drones propellers) ============
/// Used for: drone propellers, high-speed applications
#[derive(Clone, Copy, Debug)]
pub struct BrushlessCommand {
    pub rpm: u32,
}

pub trait BrushlessMotor: Send + Sync {
    fn init(&mut self) -> Result<(), MotorError>;
    fn set_rpm(&mut self, cmd: BrushlessCommand) -> Result<(), MotorError>;
    fn get_rpm(&self) -> u32;
    fn arm(&mut self) -> Result<(), MotorError>;
    fn disarm(&mut self) -> Result<(), MotorError>;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ MOTOR IMPLEMENTATIONS ============

/// Simple PWM DC Motor Implementation
pub struct PwmDcMotor {
    speed: i16,
    healthy: bool,
}

impl PwmDcMotor {
    pub fn new() -> Self {
        Self {
            speed: 0,
            healthy: false,
        }
    }
}

impl DcMotor for PwmDcMotor {
    fn init(&mut self) -> Result<(), MotorError> {
        self.healthy = true;
        Ok(())
    }

    fn set_speed(&mut self, cmd: DcMotorCommand) -> Result<(), MotorError> {
        if !self.healthy {
            return Err(MotorError::NotInitialized);
        }
        self.speed = cmd.speed.clamp(-1000, 1000);
        Ok(())
    }

    fn get_speed(&self) -> i16 {
        self.speed
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "PWM DC Motor"
    }
}

/// Simple Servo Motor Implementation
pub struct AnalogServo {
    angle: f32,
    healthy: bool,
}

impl AnalogServo {
    pub fn new() -> Self {
        Self {
            angle: 90.0,
            healthy: false,
        }
    }
}

impl ServoMotor for AnalogServo {
    fn init(&mut self) -> Result<(), MotorError> {
        self.healthy = true;
        self.angle = 90.0;
        Ok(())
    }

    fn set_angle(&mut self, cmd: ServoCommand) -> Result<(), MotorError> {
        if !self.healthy {
            return Err(MotorError::NotInitialized);
        }
        self.angle = cmd.angle_deg.clamp(0.0, 180.0);
        Ok(())
    }

    fn get_position(&self) -> f32 {
        self.angle
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "Analog Servo"
    }
}

/// Simple Stepper Motor Implementation
pub struct Stepper28Byj {
    position: i32,
    healthy: bool,
}

impl Stepper28Byj {
    pub fn new() -> Self {
        Self {
            position: 0,
            healthy: false,
        }
    }
}

impl StepperMotor for Stepper28Byj {
    fn init(&mut self) -> Result<(), MotorError> {
        self.healthy = true;
        self.position = 0;
        Ok(())
    }

    fn step(&mut self, cmd: StepperCommand) -> Result<(), MotorError> {
        if !self.healthy {
            return Err(MotorError::NotInitialized);
        }
        let steps = if cmd.direction { cmd.steps } else { -cmd.steps };
        self.position += steps;
        Ok(())
    }

    fn get_position(&self) -> i32 {
        self.position
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "28BYJ Stepper"
    }
}

/// Simple Brushless Motor Implementation (ESC-based)
pub struct Esc600kv {
    rpm: u32,
    armed: bool,
    healthy: bool,
}

impl Esc600kv {
    pub fn new() -> Self {
        Self {
            rpm: 0,
            armed: false,
            healthy: false,
        }
    }
}

impl BrushlessMotor for Esc600kv {
    fn init(&mut self) -> Result<(), MotorError> {
        self.healthy = true;
        Ok(())
    }

    fn set_rpm(&mut self, cmd: BrushlessCommand) -> Result<(), MotorError> {
        if !self.healthy {
            return Err(MotorError::NotInitialized);
        }
        if !self.armed {
            return Err(MotorError::NotArmed);
        }
        self.rpm = cmd.rpm.clamp(0, 50000);
        Ok(())
    }

    fn get_rpm(&self) -> u32 {
        self.rpm
    }

    fn arm(&mut self) -> Result<(), MotorError> {
        if !self.healthy {
            return Err(MotorError::NotInitialized);
        }
        self.armed = true;
        self.rpm = 0;
        Ok(())
    }

    fn disarm(&mut self) -> Result<(), MotorError> {
        self.armed = false;
        self.rpm = 0;
        Ok(())
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "600KV Brushless ESC"
    }
}

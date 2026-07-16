#![no_std]
use core::fmt::Debug;

/// ============ CONTROL SYSTEM ARCHITECTURE ============
/// This is the CORE of the OS - manages all vehicle behaviors

/// Control Loop Frequency
pub const CONTROL_LOOP_HZ: u32 = 1000;  // 1kHz control loop
pub const SAFETY_LOOP_HZ: u32 = 100;    // 100Hz safety monitoring

/// ============ STATE ESTIMATION ============
#[derive(Debug, Clone, Copy)]
pub struct State {
    // Position (meters)
    pub position: [f32; 3], // x, y, z
    
    // Velocity (m/s)
    pub velocity: [f32; 3],
    
    // Attitude (radians) - roll, pitch, yaw
    pub attitude: [f32; 3],
    
    // Angular velocity (rad/s)
    pub angular_velocity: [f32; 3],
    
    // Acceleration (m/s²)
    pub acceleration: [f32; 3],
}

impl State {
    pub fn new() -> Self {
        Self {
            position: [0.0; 3],
            velocity: [0.0; 3],
            attitude: [0.0; 3],
            angular_velocity: [0.0; 3],
            acceleration: [0.0; 3],
        }
    }
}

/// ============ EXTENDED KALMAN FILTER (EKF) ============
/// 16-state EKF for sensor fusion
pub struct Ekf16State {
    state: [f32; 16],
    covariance: [f32; 16],
    converged: bool,
}

impl Ekf16State {
    pub fn new() -> Self {
        Self {
            state: [0.0; 16],
            covariance: [1.0; 16],
            converged: false,
        }
    }

    pub fn predict(&mut self, dt: f32) {
        // State prediction step
        for i in 0..16 {
            self.covariance[i] += 0.001 * dt;
        }
    }

    pub fn update(&mut self, measurement: [f32; 3]) {
        // Measurement update step
        for i in 0..3 {
            self.state[i] = measurement[i];
        }
        self.converged = true;
    }

    pub fn get_state(&self) -> &[f32; 16] {
        &self.state
    }
}

/// ============ PID CONTROLLER ============
/// With anti-windup for safety
pub struct PidController {
    kp: f32,
    ki: f32,
    kd: f32,
    integral: f32,
    prev_error: f32,
    max_output: f32,
    integral_limit: f32,
}

impl PidController {
    pub fn new(kp: f32, ki: f32, kd: f32, max_output: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
            max_output,
            integral_limit: max_output * 0.5,
        }
    }

    pub fn update(&mut self, error: f32, dt: f32) -> f32 {
        // Proportional term
        let p = self.kp * error;

        // Integral term with anti-windup
        self.integral += error * dt * self.ki;
        if self.integral.abs() > self.integral_limit {
            self.integral = self.integral.signum() * self.integral_limit;
        }
        let i = self.integral;

        // Derivative term
        let derivative = if dt > 0.0 {
            (error - self.prev_error) / dt
        } else {
            0.0
        };
        let d = self.kd * derivative;

        self.prev_error = error;

        // Output
        let output = p + i + d;
        output.clamp(-self.max_output, self.max_output)
    }

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = 0.0;
    }
}

/// ============ FAILSAFE SYSTEM ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailsafeMode {
    Armed,
    WarningLevel1, // Minor issue detected
    WarningLevel2, // Significant issue
    EmergencyLand, // Immediate landing
    EmergencyStop, // Complete stop
}

pub struct FailsafeMonitor {
    mode: FailsafeMode,
    battery_voltage_min_mv: u32,
    gyro_drift_threshold: f32,
    accelerometer_threshold: f32,
    timeout_ms: u32,
}

impl FailsafeMonitor {
    pub fn new() -> Self {
        Self {
            mode: FailsafeMode::Armed,
            battery_voltage_min_mv: 9000, // 9V minimum
            gyro_drift_threshold: 0.5,
            accelerometer_threshold: 3.0,
            timeout_ms: 5000,
        }
    }

    pub fn check_battery(&mut self, voltage_mv: u32) -> FailsafeMode {
        if voltage_mv < self.battery_voltage_min_mv {
            self.mode = FailsafeMode::WarningLevel2;
        }
        self.mode
    }

    pub fn check_imu(&mut self, gyro: [f32; 3], accel: [f32; 3]) -> FailsafeMode {
        let gyro_mag = (gyro[0].powi(2) + gyro[1].powi(2) + gyro[2].powi(2)).sqrt();
        let accel_mag = (accel[0].powi(2) + accel[1].powi(2) + accel[2].powi(2)).sqrt();

        if gyro_mag > self.gyro_drift_threshold || accel_mag > self.accelerometer_threshold {
            self.mode = FailsafeMode::WarningLevel1;
        }
        self.mode
    }

    pub fn trigger_emergency(&mut self) {
        self.mode = FailsafeMode::EmergencyStop;
    }

    pub fn get_mode(&self) -> FailsafeMode {
        self.mode
    }
}

/// ============ WATCHDOG TIMER ============
pub struct Watchdog {
    timeout_ms: u32,
    last_pet_ms: u32,
    triggered: bool,
}

impl Watchdog {
    pub fn new(timeout_ms: u32) -> Self {
        Self {
            timeout_ms,
            last_pet_ms: 0,
            triggered: false,
        }
    }

    pub fn pet(&mut self, current_time_ms: u32) {
        self.last_pet_ms = current_time_ms;
        self.triggered = false;
    }

    pub fn check(&mut self, current_time_ms: u32) -> bool {
        if current_time_ms - self.last_pet_ms > self.timeout_ms {
            self.triggered = true;
            true
        } else {
            false
        }
    }

    pub fn is_triggered(&self) -> bool {
        self.triggered
    }
}

/// ============ MISSION PLANNER ============
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionState {
    Idle,
    Planning,
    InProgress,
    Paused,
    Completed,
    Failed,
}

pub struct Waypoint {
    pub position: [f32; 3], // x, y, z
    pub heading: f32,
    pub wait_time_ms: u32,
}

pub struct MissionPlanner {
    waypoints: [Option<Waypoint>; 32],
    current_waypoint: u8,
    state: MissionState,
    total_waypoints: u8,
}

impl MissionPlanner {
    pub fn new() -> Self {
        Self {
            waypoints: [None; 32],
            current_waypoint: 0,
            state: MissionState::Idle,
            total_waypoints: 0,
        }
    }

    pub fn add_waypoint(&mut self, wp: Waypoint) -> Result<(), ()> {
        if self.total_waypoints >= 32 {
            return Err(());
        }
        self.waypoints[self.total_waypoints as usize] = Some(wp);
        self.total_waypoints += 1;
        Ok(())
    }

    pub fn start(&mut self) {
        if self.total_waypoints > 0 {
            self.state = MissionState::InProgress;
            self.current_waypoint = 0;
        }
    }

    pub fn next_waypoint(&mut self) {
        if self.current_waypoint < self.total_waypoints - 1 {
            self.current_waypoint += 1;
        } else {
            self.state = MissionState::Completed;
        }
    }

    pub fn get_current_waypoint(&self) -> Option<Waypoint> {
        self.waypoints[self.current_waypoint as usize]
    }

    pub fn abort(&mut self) {
        self.state = MissionState::Failed;
    }
}

/// ============ CONTROL SYSTEM ============
pub struct ControlSystem {
    pub state_estimator: Ekf16State,
    pub pid_roll: PidController,
    pub pid_pitch: PidController,
    pub pid_yaw: PidController,
    pub pid_altitude: PidController,
    pub pid_velocity_x: PidController,
    pub pid_velocity_y: PidController,
    pub failsafe: FailsafeMonitor,
    pub watchdog: Watchdog,
    pub mission: MissionPlanner,
}

impl ControlSystem {
    pub fn new() -> Self {
        Self {
            state_estimator: Ekf16State::new(),
            pid_roll: PidController::new(4.5, 0.1, 0.15, 500.0),
            pid_pitch: PidController::new(4.5, 0.1, 0.15, 500.0),
            pid_yaw: PidController::new(4.5, 0.05, 0.1, 500.0),
            pid_altitude: PidController::new(1.0, 0.02, 0.2, 1000.0),
            pid_velocity_x: PidController::new(0.5, 0.01, 0.1, 500.0),
            pid_velocity_y: PidController::new(0.5, 0.01, 0.1, 500.0),
            failsafe: FailsafeMonitor::new(),
            watchdog: Watchdog::new(100),
            mission: MissionPlanner::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        // Initialize EKF
        self.state_estimator = Ekf16State::new();
        
        // Initialize watchdog
        self.watchdog.pet(0);
        
        Ok(())
    }
}

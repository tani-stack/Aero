#![allow(dead_code)]
use core::fmt::Debug;

/// Generic Sensor Error for all vehicle types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorError {
    NotReady,
    BusFault,
    Timeout,
    CalibrationNeeded,
    NotInitialized,
}

/// ============ ULTRASONIC SENSOR (Robots, Cars) ============
/// Used for: obstacle detection, distance measurement
#[derive(Clone, Copy, Debug)]
pub struct DistanceMeasurement {
    pub distance_cm: f32,
    pub timestamp_ms: u64,
}

pub trait DistanceSensor: Send + Sync {
    fn init(&mut self) -> Result<(), SensorError>;
    fn read(&mut self) -> Result<DistanceMeasurement, SensorError>;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ ENCODER SENSOR (Cars, Robots) ============
/// Used for: speed measurement, odometry, position tracking
#[derive(Clone, Copy, Debug)]
pub struct EncoderReading {
    pub ticks: u32,
    pub rpm: f32,
}

pub trait Encoder: Send + Sync {
    fn init(&mut self) -> Result<(), SensorError>;
    fn read(&mut self) -> Result<EncoderReading, SensorError>;
    fn reset(&mut self) -> Result<(), SensorError>;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ TEMPERATURE SENSOR (All vehicles) ============
/// Used for: motor temperature, battery temperature, environmental
#[derive(Clone, Copy, Debug)]
pub struct TemperatureReading {
    pub temp_celsius: f32,
}

pub trait TemperatureSensor: Send + Sync {
    fn init(&mut self) -> Result<(), SensorError>;
    fn read(&mut self) -> Result<TemperatureReading, SensorError>;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ CURRENT SENSOR (All vehicles) ============
/// Used for: power monitoring, fault detection
#[derive(Clone, Copy, Debug)]
pub struct CurrentReading {
    pub current_ma: f32,
}

pub trait CurrentSensor: Send + Sync {
    fn init(&mut self) -> Result<(), SensorError>;
    fn read(&mut self) -> Result<CurrentReading, SensorError>;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ LIMIT SWITCH (Robots) ============
/// Used for: end-of-travel detection, safety limits
#[derive(Clone, Copy, Debug)]
pub struct SwitchState {
    pub triggered: bool,
}

pub trait LimitSwitch: Send + Sync {
    fn init(&mut self) -> Result<(), SensorError>;
    fn read(&mut self) -> Result<SwitchState, SensorError>;
    fn is_healthy(&self) -> bool;
    fn name(&self) -> &'static str;
}

/// ============ PRESSURE SENSOR (Drones - altitude) ============
/// Already in barometer.rs, keeping reference

/// ============ SENSOR IMPLEMENTATIONS ============

/// Simple Ultrasonic Sensor Implementation
pub struct Hc_Sr04 {
    distance_cm: f32,
    healthy: bool,
}

impl Hc_Sr04 {
    pub fn new() -> Self {
        Self {
            distance_cm: 0.0,
            healthy: false,
        }
    }
}

impl DistanceSensor for Hc_Sr04 {
    fn init(&mut self) -> Result<(), SensorError> {
        self.healthy = true;
        Ok(())
    }

    fn read(&mut self) -> Result<DistanceMeasurement, SensorError> {
        if !self.healthy {
            return Err(SensorError::NotInitialized);
        }
        Ok(DistanceMeasurement {
            distance_cm: 42.5,
            timestamp_ms: 0,
        })
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "HC-SR04 Ultrasonic"
    }
}

/// Rotary Encoder Implementation
pub struct RotaryEncoder {
    ticks: u32,
    rpm: f32,
    healthy: bool,
}

impl RotaryEncoder {
    pub fn new() -> Self {
        Self {
            ticks: 0,
            rpm: 0.0,
            healthy: false,
        }
    }
}

impl Encoder for RotaryEncoder {
    fn init(&mut self) -> Result<(), SensorError> {
        self.healthy = true;
        self.ticks = 0;
        Ok(())
    }

    fn read(&mut self) -> Result<EncoderReading, SensorError> {
        if !self.healthy {
            return Err(SensorError::NotInitialized);
        }
        Ok(EncoderReading {
            ticks: self.ticks,
            rpm: self.rpm,
        })
    }

    fn reset(&mut self) -> Result<(), SensorError> {
        self.ticks = 0;
        Ok(())
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "Rotary Encoder"
    }
}

/// Temperature Sensor Implementation (NTC Thermistor)
pub struct NtcThermistor {
    temp: f32,
    healthy: bool,
}

impl NtcThermistor {
    pub fn new() -> Self {
        Self {
            temp: 25.0,
            healthy: false,
        }
    }
}

impl TemperatureSensor for NtcThermistor {
    fn init(&mut self) -> Result<(), SensorError> {
        self.healthy = true;
        Ok(())
    }

    fn read(&mut self) -> Result<TemperatureReading, SensorError> {
        if !self.healthy {
            return Err(SensorError::NotInitialized);
        }
        Ok(TemperatureReading {
            temp_celsius: self.temp,
        })
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "NTC Thermistor"
    }
}

/// Current Sensor Implementation (ACS712)
pub struct Acs712 {
    current_ma: f32,
    healthy: bool,
}

impl Acs712 {
    pub fn new() -> Self {
        Self {
            current_ma: 0.0,
            healthy: false,
        }
    }
}

impl CurrentSensor for Acs712 {
    fn init(&mut self) -> Result<(), SensorError> {
        self.healthy = true;
        Ok(())
    }

    fn read(&mut self) -> Result<CurrentReading, SensorError> {
        if !self.healthy {
            return Err(SensorError::NotInitialized);
        }
        Ok(CurrentReading {
            current_ma: self.current_ma,
        })
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "ACS712 Current Sensor"
    }
}

/// Limit Switch Implementation
pub struct MechanicalSwitch {
    triggered: bool,
    healthy: bool,
}

impl MechanicalSwitch {
    pub fn new() -> Self {
        Self {
            triggered: false,
            healthy: false,
        }
    }
}

impl LimitSwitch for MechanicalSwitch {
    fn init(&mut self) -> Result<(), SensorError> {
        self.healthy = true;
        Ok(())
    }

    fn read(&mut self) -> Result<SwitchState, SensorError> {
        if !self.healthy {
            return Err(SensorError::NotInitialized);
        }
        Ok(SwitchState {
            triggered: self.triggered,
        })
    }

    fn is_healthy(&self) -> bool {
        self.healthy
    }

    fn name(&self) -> &'static str {
        "Mechanical Limit Switch"
    }
}

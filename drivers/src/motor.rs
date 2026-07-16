use bitflags::bitflags;

#[derive(Clone, Copy, Debug)]
pub struct MotorTelemetry {
    pub rpm: u32,
    pub current_ma: u32,
    pub temp_c: i16,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct MotorFault: u8 {
        const OVERCURRENT = 1;
        const STALL = 2;
        const OVERTEMP = 4;
    }
}

#[derive(Debug)]
pub enum MotorError {
    NotArmed,
    InvalidRpm,
    BusFault,
}

pub trait BrushlessMotor: Send + Sync {
    fn id(&self) -> u8;
    fn arm(&mut self) -> Result<(), MotorError>;
    fn set_rpm(&mut self, rpm: u32) -> Result<(), MotorError>;
    fn brake(&mut self) -> Result<(), MotorError>;
    fn telemetry(&self) -> MotorTelemetry;
    fn is_healthy(&self) -> bool {
        true
    }
}

pub struct DshotMotor<B> {
    pub bus: B,
    pub mid: u8,
    pub telem: MotorTelemetry,
}

impl<B: crate::bus::spi::SpiBus> DshotMotor<B> {
    pub fn new(bus: B, id: u8) -> Self {
        Self {
            bus,
            mid: id,
            telem: MotorTelemetry {
                rpm: 0,
                current_ma: 0,
                temp_c: 25,
            },
        }
    }
    fn dshot_checksum(v: u16) -> u8 {
        let mut c = (v ^ (v >> 4) ^ (v >> 8)) as u8;
        c & 0x0F
    }
}

impl<B: crate::bus::spi::SpiBus + Send + Sync> BrushlessMotor for DshotMotor<B> {
    fn id(&self) -> u8 {
        self.mid
    }
    fn arm(&mut self) -> Result<(), MotorError> {
        let pkt: u16 = 0;
        let cs = Self::dshot_checksum(pkt) as u16;
        let frame = [(pkt >> 8) as u8, (pkt as u8 & 0xF0) | (cs as u8 & 0x0F)];
        self.bus
            .transfer(&frame, &mut [])
            .map_err(|_| MotorError::BusFault)
    }
    fn set_rpm(&mut self, rpm: u32) -> Result<(), MotorError> {
        if rpm > 25000 {
            return Err(MotorError::InvalidRpm);
        }
        let throttle = ((rpm as f32 / 25000.0) * 1999.0) as u16 + 48;
        let cs = Self::dshot_checksum(throttle << 1);
        let frame = [(throttle >> 3) as u8, ((throttle << 5) as u8 | (cs & 0x0F))];
        self.bus
            .transfer(&frame, &mut [])
            .map_err(|_| MotorError::BusFault)?;
        self.telem.rpm = rpm;
        Ok(())
    }
    fn brake(&mut self) -> Result<(), MotorError> {
        self.set_rpm(0)
    }
    fn telemetry(&self) -> MotorTelemetry {
        self.telem
    }
}

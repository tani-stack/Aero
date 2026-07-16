#[derive(Clone, Copy, Debug)]
pub struct BaroData {
    pub pressure_pa: f32,
    pub temp_c: f32,
    pub alt_m: f32,
}

pub trait Barometer: Send + Sync {
    fn init(&mut self) -> Result<(), ()>;
    fn read(&mut self) -> BaroData;
}

pub struct Bmp390<B> {
    bus: B,
}

impl<B> Bmp390<B> {
    pub fn new(b: B) -> Self {
        Self { bus: b }
    }
}

impl<B: crate::bus::i2c::I2cBus + Send + Sync> Barometer for Bmp390<B> {
    fn init(&mut self) -> Result<(), ()> {
        Ok(())
    }
    fn read(&mut self) -> BaroData {
        BaroData {
            pressure_pa: 101325.0,
            temp_c: 25.0,
            alt_m: 0.0,
        }
    }
}

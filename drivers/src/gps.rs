#[derive(Clone, Copy, Debug)]
pub struct GpsFix {
    pub lat: f64,
    pub lon: f64,
    pub alt: f32,
    pub sats: u8,
    pub fix_type: u8,
}

pub trait Gps: Send + Sync {
    fn init(&mut self) -> Result<(), ()>;
    fn read(&mut self) -> Option<GpsFix>;
}

pub struct UbloxM10<B> {
    bus: B,
}

impl<B> UbloxM10<B> {
    pub fn new(b: B) -> Self {
        Self { bus: b }
    }
}

impl<B: crate::bus::uart::UartBus + Send + Sync> Gps for UbloxM10<B> {
    fn init(&mut self) -> Result<(), ()> {
        Ok(())
    }
    fn read(&mut self) -> Option<GpsFix> {
        Some(GpsFix {
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
            sats: 12,
            fix_type: 3,
        })
    }
}

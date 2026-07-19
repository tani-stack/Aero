#[derive(Debug, Clone, Copy)]
pub struct CanFrame {
    pub id: u32,
    pub dlc: u8,
    pub data: [u8; 8],
}

#[derive(Debug)]
pub enum CanError {
    BusOff,
    Overrun,
    InvalidDlc,
}

pub trait CanBus {
    fn send(&mut self, frame: &CanFrame) -> Result<(), CanError>;
    fn recv(&mut self) -> Result<Option<CanFrame>, CanError>;
}

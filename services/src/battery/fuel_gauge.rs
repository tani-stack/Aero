pub trait FuelGauge {
    fn soc(&mut self) -> u8;
    fn voltage_mv(&mut self) -> u32;
    fn current_ma(&mut self) -> i32;
}

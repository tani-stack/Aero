pub trait SpiBus {
    fn transfer(&mut self, tx: &[u8], rx: &mut [u8]) -> Result<(), ()>;
    fn write(&mut self, data: &[u8]) -> Result<(), ()> {
        let mut rx = [0u8; 0];
        self.transfer(data, &mut rx)
    }
}

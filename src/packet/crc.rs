
/// Implementation from
/// https://resources.basicmicro.com/cyclic-redundancy-check-explained/.
#[derive(Clone)]
pub struct CRC16 {
    pub state: u16,
    pub poly: u16,
}

impl CRC16 {
    pub const fn new(init: u16, poly: u16) -> Self { Self { state: init, poly } }

    pub fn resume(&mut self, byte: u8) {
        self.state ^= (byte as u16) << 8;
        for _ in 0..8 {
            if (self.state & 0x8000) != 0 {
                self.state = (self.state << 1) ^ self.poly;
            } else {
                self.state = self.state << 1;
            }
        }
    }

    pub const fn complete(self) -> u16 { self.state }
}

impl Default for CRC16 {
    fn default() -> Self { Self::new(0, 0x1021) }
}

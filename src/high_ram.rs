pub struct HighRam(Box<[u8]>);

impl HighRam {
    pub fn new() -> Self {
        Self(Box::new([0; 0x80])) // 0x80 = 128 (bytes)
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[(addr as usize) & 0x7f]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.0[(addr as usize) & 0x7f] = val;
    }
}

impl Default for HighRam {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BootRom {
    rom: Box<[u8]>,
    is_active: bool,
}

impl BootRom {
    pub fn new(rom: Box<[u8]>) -> Self {
        Self {
            rom,
            is_active: true,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    pub fn write(&mut self, _: u16, val: u8) {
        self.is_active &= val == 0;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

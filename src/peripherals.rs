use crate::boot_rom::BootRom;
use crate::high_ram::HighRam;
use crate::work_ram::WorkRam;

pub struct Peripherals {
    boot_rom: BootRom,
    work_ram: WorkRam,
    high_ram: HighRam,
}

impl Peripherals {
    pub fn new(boot_rom: BootRom) -> Self {
        Self {
            boot_rom,
            work_ram: WorkRam::new(),
            high_ram: HighRam::new(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00ff => {
                if self.boot_rom.is_active() {
                    self.boot_rom.read(addr)
                } else {
                    0xff // TODO: Read from cartridge
                }
            }
            0xc000..=0xfdff => self.work_ram.read(addr),
            0xff80..=0xfffe => self.high_ram.read(addr),
            _ => 0xff, // TODO: Unimplemented
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xc000..=0xfdff => self.work_ram.write(addr, val),
            0xff50 => self.boot_rom.write(addr, val),
            0xff80..=0xfffe => self.high_ram.write(addr, val),
            _ => (), // TODO: Unimplemented
        }
    }
}

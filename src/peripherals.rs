mod boot_rom;
mod high_ram;
mod ppu;
mod work_ram;

pub use boot_rom::BootRom;
use high_ram::HighRam;
use ppu::Ppu;
use work_ram::WorkRam;

pub struct Peripherals {
    boot_rom: BootRom,
    work_ram: WorkRam,
    high_ram: HighRam,
    ppu: Ppu,
}

impl Peripherals {
    pub fn new(boot_rom: BootRom) -> Self {
        Self {
            boot_rom,
            work_ram: WorkRam::new(),
            high_ram: HighRam::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn emulate_ppu_cycle(&mut self) -> bool {
        self.ppu.emulate_cycle()
    }

    pub fn pixel_buffer(&self) -> Box<[u8]> {
        self.ppu.pixel_buffer()
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
            0x8000..=0x9fff => self.ppu.read(addr),
            0xfe00..=0xfe9f => self.ppu.read(addr),
            0xff40..=0xff4b => self.ppu.read(addr),
            0xc000..=0xfdff => self.work_ram.read(addr),
            0xff80..=0xfffe => self.high_ram.read(addr),
            _ => 0xff, // TODO: Unimplemented
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9fff => self.ppu.write(addr, val),
            0xfe00..=0xfe9f => self.ppu.write(addr, val),
            0xff40..=0xff4b => self.ppu.write(addr, val),
            0xc000..=0xfdff => self.work_ram.write(addr, val),
            0xff50 => self.boot_rom.write(addr, val),
            0xff80..=0xfffe => self.high_ram.write(addr, val),
            _ => (), // TODO: Unimplemented
        }
    }
}

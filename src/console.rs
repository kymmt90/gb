use std::time;

use crate::{
    cpu::Cpu,
    peripherals::{BootRom, Peripherals},
};

pub struct Console {
    cpu: Cpu,
    peripherals: Peripherals,
}

impl Console {
    pub const CPU_CLOCK_HZ: u128 = 4_194_304;
    pub const M_CYCLE_CLOCK: u128 = 4;
    const M_CYCLE_NANOS: u128 = Self::M_CYCLE_CLOCK * 1_000_000_000 / Self::CPU_CLOCK_HZ;

    pub fn new(boot_rom: BootRom) -> Self {
        let peripherals = Peripherals::new(boot_rom);
        let cpu = Cpu::new();

        Self { cpu, peripherals }
    }

    pub fn run(&mut self) {
        let time = time::Instant::now();
        let mut elapsed = 0;

        loop {
            let e = time.elapsed().as_nanos();
            for _ in 0..(e - elapsed) / Self::M_CYCLE_NANOS {
                self.cpu.emulate_cycle(&mut self.peripherals);

                if self.peripherals.emulate_ppu_cycle() {
                    // self.lcd.draw(self.peripherals.pixel_buffer());
                }

                elapsed += Self::M_CYCLE_NANOS;
            }
        }
    }
}

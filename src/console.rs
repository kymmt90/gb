use std::time;

use crate::{
    cpu::Cpu,
    lcd::Lcd,
    peripherals::{BootRom, Peripherals},
};

pub struct Console {
    cpu: Cpu,
    peripherals: Peripherals,
    lcd: Lcd,
    sdl: sdl2::Sdl,
}

impl Console {
    pub const CPU_CLOCK_HZ: u128 = 4_194_304;
    pub const M_CYCLE_CLOCK: u128 = 4;
    const M_CYCLE_NANOS: u128 = Self::M_CYCLE_CLOCK * 1_000_000_000 / Self::CPU_CLOCK_HZ;

    pub fn new(boot_rom: BootRom) -> Self {
        let peripherals = Peripherals::new(boot_rom);
        let cpu = Cpu::new();

        let sdl = sdl2::init().expect("failed to initialize SDL");
        let lcd = Lcd::new(&sdl, 4);

        Self {
            cpu,
            peripherals,
            lcd,
            sdl,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();

        let time = time::Instant::now();
        let mut elapsed = 0;

        'running: loop {
            let e = time.elapsed().as_nanos();

            for _ in 0..(e - elapsed) / Self::M_CYCLE_NANOS {
                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'running,
                        sdl2::event::Event::KeyDown {
                            keycode: Some(k), ..
                        } => {
                            if k == sdl2::keyboard::Keycode::Escape {
                                break 'running;
                            }
                        }
                        _ => (),
                    }
                }

                self.cpu.emulate_cycle(&mut self.peripherals);

                if self.peripherals.emulate_ppu_cycle() {
                    self.lcd.draw(self.peripherals.pixel_buffer());
                }

                elapsed += Self::M_CYCLE_NANOS;
            }
        }
    }
}

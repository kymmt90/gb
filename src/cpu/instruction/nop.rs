use crate::{cpu::Cpu, peripherals::Peripherals};

pub trait Nop {
    fn nop(&mut self, bus: &Peripherals);
}

impl Nop for Cpu {
    fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }
}

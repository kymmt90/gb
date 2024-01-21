use crate::{
    cpu::{
        instruction::Push as _,
        operand::{go, step, Imm16, IO16 as _},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

trait Call {
    fn call(&mut self, bus: &mut Peripherals);
}

impl Call for Cpu {
    fn call(&mut self, bus: &mut Peripherals) {
        step!((), {
            0: if let Some(v) = self.read16(bus, Imm16) {
                VAL16.store(v, Relaxed);

                go!(1);
            },
            1: if self.push16(bus, self.regs.pc).is_some() {
                self.regs.pc = VAL16.load(Relaxed);

                go!(0);

                self.fetch(bus);
            },
        });
    }
}

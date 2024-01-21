use crate::{
    cpu::{
        instruction::Pop as _,
        operand::{go, step},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

pub trait Ret {
    fn ret(&mut self, bus: &mut Peripherals);
}

impl Ret for Cpu {
    fn ret(&mut self, bus: &mut Peripherals) {
        step!((), {
            0: if let Some(v) = self.pop16(bus) {
                self.regs.pc = v;

                go!(1);
            },
            1: {
                go!(0);

                self.fetch(bus);
            },
        });
    }
}

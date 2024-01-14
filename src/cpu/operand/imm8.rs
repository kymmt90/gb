use crate::{
    cpu::{
        operand::{go, step, IO8},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

#[derive(Debug, Copy, Clone)]
pub struct Imm8;

impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, _: Imm8) -> Option<u8> {
        step!(None, {
            0: {
                VAL8.store(bus.read(self.regs.pc), Relaxed);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                go!(1);

                return None;
            },
            1: {
                go!(0);

                #[allow(clippy::needless_return)]
                return Some(VAL8.load(Relaxed));
            },
        });
    }

    fn write8(&mut self, _: &mut Peripherals, _: Imm8, _: u8) -> Option<()> {
        unreachable!()
    }
}

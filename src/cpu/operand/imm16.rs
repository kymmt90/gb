use crate::{
    cpu::{
        operand::{go, imm8::Imm8, step, IO16, IO8 as _},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

#[derive(Debug, Copy, Clone)]
pub struct Imm16;

impl IO16<Imm16> for Cpu {
    fn read16(&mut self, bus: &Peripherals, _: Imm16) -> Option<u16> {
        step!(None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                VAL8.store(lo, Relaxed);

                go!(1);
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);

                go!(2);
            },
            2: {
                go!(0);

                #[allow(clippy::needless_return)]
                return Some(VAL16.load(Relaxed));
            },
        });
    }

    fn write16(&mut self, _: &mut Peripherals, _: Imm16, _: u16) -> Option<()> {
        unreachable!()
    }
}

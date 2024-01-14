use crate::{
    cpu::{
        operand::{go, imm8::Imm8, step, IO16, IO8},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

#[derive(Debug, Copy, Clone)]
pub struct Direct16;

impl IO16<Direct16> for Cpu {
    fn read16(&mut self, _: &Peripherals, _: Direct16) -> Option<u16> {
        unreachable!()
    }

    fn write16(&mut self, bus: &mut Peripherals, _: Direct16, val: u16) -> Option<()> {
        step!(
            None, {
                0: if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, Relaxed);

                    go!(1);
                },
                1: if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);

                    go!(2);
                },
                2: {
                    bus.write(VAL16.load(Relaxed), val as u8);

                    go!(3);

                    return None;
                },
                3: {
                    bus.write(VAL16.load(Relaxed).wrapping_add(1), (val >> 8) as u8);

                    go!(4);

                    return None;
                },
                4: #[allow(clippy::needless_return)] return Some(go!(0)),
            }
        );
    }
}

use crate::{
    cpu::{
        operand::{go, imm8::Imm8, step, IO8},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Direct8 {
    D,
    DFF,
}

impl IO8<Direct8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Direct8) -> Option<u8> {
        step!(
            None, {
                0: if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, Relaxed);

                    go!(1);

                    if let Direct8::DFF = src {
                        VAL16.store(0xff00 | (lo as u16), Relaxed);

                        go!(2);
                    }
                },
                1: if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);

                    go!(2);
                },
                2: {
                    VAL8.store(bus.read(VAL16.load(Relaxed)), Relaxed);

                    go!(3);

                    return None;
                },
                3: {
                    go!(0);

                    #[allow(clippy::needless_return)]
                    return Some(VAL8.load(Relaxed));
                },
            }
        );
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Direct8, val: u8) -> Option<()> {
        step!(
            None, {
                0: if let Some(lo) = self.read8(bus, Imm8) {
                    VAL8.store(lo, Relaxed);

                    go!(1);

                    if let Direct8::DFF = dst {
                        VAL16.store(0xff00 | (lo as u16), Relaxed);

                        go!(2);
                    }
                },
                1: if let Some(hi) = self.read8(bus, Imm8) {
                    VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);

                    go!(2);
                },
                2: {
                    bus.write(VAL16.load(Relaxed), val);

                    go!(3);

                    return None;
                },
                3: #[allow(clippy::needless_return)] return Some(go!(0)),
            }
        );
    }
}

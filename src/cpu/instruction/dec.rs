use crate::{
    cpu::{
        operand::{go, step, IO16, IO8},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

pub trait Dec {
    fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>;

    fn dec16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO16<S>;
}

impl Dec for Cpu {
    fn dec<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        step!((), {
            0: if let Some(val) = self.read8(bus, src) {
                let result = val.wrapping_sub(1);

                self.regs.set_zf(result == 0);
                self.regs.set_nf(true);
                self.regs.set_hf((val & 0xf) == 0);

                VAL8.store(result, Relaxed);

                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                go!(0);

                self.fetch(bus);
            },
        });
    }

    fn dec16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO16<S>,
    {
        step!((), {
            0: if let Some(val) = self.read16(bus, src) {
                VAL16.store(val.wrapping_sub(1), Relaxed);

                go!(1);
            },
            1: if self.write16(bus, src, VAL16.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(0);

                self.fetch(bus);
            },
        });
    }
}

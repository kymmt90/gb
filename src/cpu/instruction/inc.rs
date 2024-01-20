use crate::{
    cpu::{
        operand::{go, step, IO16, IO8},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

trait Inc {
    fn inc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>;

    fn inc16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO16<S>;
}

impl Inc for Cpu {
    fn inc<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        step!((), {
            0: if let Some(val) = self.read8(bus, src) {
                let result = val.wrapping_add(1);

                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf((val & 0xf) == 0xf);

                VAL8.store(result, Relaxed);

                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed).wrapping_add(1)).is_some() {
                go!(0);

                self.fetch(bus);
            },
        });
    }

    fn inc16<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO16<S>,
    {
        step!((), {
            0: if let Some(val) = self.read16(bus, src) {
                VAL16.store(val.wrapping_add(1), Relaxed);

                go!(1);
            },
            1: if self.write16(bus, src, VAL16.load(Relaxed).wrapping_add(1)).is_some() {
                go!(2);
            },
            2: {
                go!(0);

                self.fetch(bus);
            },
        });
    }
}

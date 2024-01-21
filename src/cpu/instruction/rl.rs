use crate::{
    cpu::{
        operand::{go, step, IO8},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

trait Rl {
    fn rl<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>;
}

impl Rl for Cpu {
    fn rl<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        step!((), {
            0: if let Some(val) = self.read8(bus, src) {
                let result = (val << 1) | self.regs.cf() as u8;

                self.regs.set_zf(result == 0);
                self.regs.set_nf(false);
                self.regs.set_hf(false);
                self.regs.set_cf(val & 0x80 > 0);

                VAL8.store(result, Relaxed);

                go!(1);
            },
            1: if self.write8(bus, src, VAL8.load(Relaxed)).is_some() {
                go!(0);

                self.fetch(bus);
            },
        });
    }
}

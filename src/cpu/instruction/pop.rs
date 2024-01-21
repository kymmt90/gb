use crate::{
    cpu::{
        operand::{go, step, Reg16, IO16},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

pub trait Pop {
    fn pop16(&mut self, bus: &mut Peripherals) -> Option<u16>;
    fn pop(&mut self, bus: &mut Peripherals, dst: Reg16);
}

impl Pop for Cpu {
    fn pop16(&mut self, bus: &mut Peripherals) -> Option<u16> {
        step!(None, {
            0: {
                VAL8.store(bus.read(self.regs.sp), Relaxed);
                self.regs.sp = self.regs.sp.wrapping_add(1);

                go!(1);

                return None;
            },
            1: {
                let hi = bus.read(self.regs.sp);
                self.regs.sp = self.regs.sp.wrapping_add(1);
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);

                go!(2);

                return None;
            },
            2: {
                go!(0);

                #[allow(clippy::needless_return)]
                return Some(VAL16.load(Relaxed));
            },
        });
    }

    fn pop(&mut self, bus: &mut Peripherals, dst: Reg16) {
        if let Some(v) = self.pop16(bus) {
            self.write16(bus, dst, v);
            self.fetch(bus);
        }
    }
}

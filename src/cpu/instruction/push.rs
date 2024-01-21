use crate::{
    cpu::{
        operand::{go, step, Reg16, IO16},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

pub trait Push {
    fn push16(&mut self, bus: &mut Peripherals, val: u16) -> Option<()>;
    fn push(&mut self, bus: &mut Peripherals, val: Reg16);
}

impl Push for Cpu {
    fn push16(&mut self, bus: &mut Peripherals, val: u16) -> Option<()> {
        step!(None, {
            0: {
                go!(1);

                return None;
            },
            1: {
                let [lo, hi] = u16::to_le_bytes(val);
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, hi);

                go!(2);

                return None;
            },
            2: {
                self.regs.sp = self.regs.sp.wrapping_sub(1);
                bus.write(self.regs.sp, VAL8.load(Relaxed));

                go!(3);

                return None;
            },
            3: return Some(go!(0)),
        });
    }

    fn push(&mut self, bus: &mut Peripherals, src: Reg16) {
        step!((), {
            0: {
                VAL16.store(self.read16(bus, src).unwrap(), Relaxed);

                go!(1);
            },
            1: if self.push16(bus, VAL16.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(0);

                self.fetch(bus);
            },
        });
    }
}

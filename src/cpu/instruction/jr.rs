use crate::{
    cpu::{
        operand::{go, step, Cond, Imm8, IO8},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

trait Jr {
    fn jr(&mut self, bus: &mut Peripherals);
    fn jr_c(&mut self, bus: &mut Peripherals, c: Cond);
    fn cond(&self, cond: Cond) -> bool;
}

impl Jr for Cpu {
    fn jr(&mut self, bus: &mut Peripherals) {
        step!((), {
            0: if let Some(v) = self.read8(bus, Imm8) {
                self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);

                go!(1);
            },
            1: {
                go!(0);

                self.fetch(bus);
            },
        });
    }

    fn jr_c(&mut self, bus: &mut Peripherals, c: Cond) {
        step!((), {
            0: if let Some(v) = self.read8(bus, Imm8) {
                go!(1);

                if self.cond(c) {
                    self.regs.pc = self.regs.pc.wrapping_add(v as i8 as u16);
                    return;
                }
            },
            1: {
                go!(0);

                self.fetch(bus);
            },
        });
    }

    fn cond(&self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.regs.nf(),
            Cond::Z => self.regs.zf(),
            Cond::NC => !self.regs.cf(),
            Cond::C => self.regs.cf(),
        }
    }
}

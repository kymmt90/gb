use crate::{
    cpu::{
        operand::{go, step, IO8},
        Cpu,
    },
    peripherals::Peripherals,
};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Indirect {
    BC,
    DE,
    HL,
    CFF,
    HLD,
    HLI,
}

impl IO8<Indirect> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Indirect) -> Option<u8> {
        step!(
            None, {
                0: {
                    VAL8.store(match src {
                        Indirect::BC => bus.read(self.regs.bc()),
                        Indirect::DE => bus.read(self.regs.de()),
                        Indirect::HL => bus.read(self.regs.hl()),
                        Indirect::CFF => bus.read(0xff00 | u16::from(self.regs.c)),
                        Indirect::HLD => {
                            let hl = self.regs.hl();
                            self.regs.set_hl(hl.wrapping_sub(1));
                            bus.read(hl)
                        },
                        Indirect::HLI => {
                            let hl = self.regs.hl();
                            self.regs.set_hl(hl.wrapping_add(1));
                            bus.read(hl)
                        },
                    }, Relaxed);

                    go!(1);

                    return None;
                },
                1: {
                    go!(0);

                    #[allow(clippy::needless_return)]
                    return Some(VAL8.load(Relaxed));
                },
            }
        );
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Indirect, val: u8) -> Option<()> {
        step!(
            None, {
                0: {
                    match dst {
                        Indirect::BC => bus.write(self.regs.bc(), val),
                        Indirect::DE => bus.write(self.regs.de(), val),
                        Indirect::HL => bus.write(self.regs.hl(), val),
                        Indirect::CFF => bus.write(0xff00 | u16::from(self.regs.c), val),
                        Indirect::HLD => {
                            let hl = self.regs.hl();
                            self.regs.set_hl(hl.wrapping_sub(1));
                            bus.write(hl, val)
                        },
                        Indirect::HLI => {
                            let hl = self.regs.hl();
                            self.regs.set_hl(hl.wrapping_add(1));
                            bus.write(hl, val)
                        },
                    }

                    go!(1);

                    return None;
                },
                1: {
                    go!(0);

                    #[allow(clippy::needless_return)]
                    return Some(());
                },
            }
        );
    }
}

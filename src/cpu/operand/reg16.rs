use crate::{
    cpu::{operand::IO16, Cpu},
    peripherals::Peripherals,
};

#[derive(Debug, Copy, Clone)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> Option<u16> {
        Some(match src {
            Reg16::AF => self.regs.af(),
            Reg16::BC => self.regs.bc(),
            Reg16::DE => self.regs.de(),
            Reg16::HL => self.regs.hl(),
            Reg16::SP => self.regs.sp,
        })
    }

    fn write16(&mut self, _: &mut Peripherals, dst: Reg16, val: u16) -> Option<()> {
        match dst {
            Reg16::AF => self.regs.set_af(val),
            Reg16::BC => self.regs.set_bc(val),
            Reg16::DE => self.regs.set_de(val),
            Reg16::HL => self.regs.set_hl(val),
            Reg16::SP => self.regs.sp = val,
        }

        Some(())
    }
}

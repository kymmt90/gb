use crate::{
    cpu::{operand::IO8, Cpu},
    peripherals::Peripherals,
};

pub trait Cp {
    fn cp<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>;
}

impl Cp for Cpu {
    fn cp<S: Copy>(&mut self, bus: &mut Peripherals, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(v) = self.read8(bus, src) {
            let (result, carry) = self.regs.a.overflowing_sub(v);

            self.regs.set_zf(result == 0);
            self.regs.set_nf(true);
            self.regs.set_hf((self.regs.a & 0xf) < (v & 0xf));
            self.regs.set_cf(carry);

            self.fetch(bus);
        }
    }
}

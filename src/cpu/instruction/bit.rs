use crate::{
    cpu::{operand::IO8, Cpu},
    peripherals::Peripherals,
};

trait Bit {
    fn bit<S: Copy>(&mut self, bus: &mut Peripherals, bit: usize, src: S)
    where
        Self: IO8<S>;
}

impl Bit for Cpu {
    fn bit<S: Copy>(&mut self, bus: &mut Peripherals, bit: usize, src: S)
    where
        Self: IO8<S>,
    {
        if let Some(val) = self.read8(bus, src) {
            self.regs.set_zf(val & (1 << bit) == 0);
            self.regs.set_nf(false);
            self.regs.set_hf(true);

            self.fetch(bus);
        }
    }
}

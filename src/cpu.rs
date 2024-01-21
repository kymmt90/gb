mod decode;
mod instruction;
mod operand;
mod registers;

use crate::peripherals::Peripherals;
use decode::Decode as _;

#[derive(Debug, Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}

#[derive(Debug, Default)]
pub struct Cpu {
    regs: registers::Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }

    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }
}

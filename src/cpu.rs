mod operand;
mod registers;

use crate::peripherals::Peripherals;

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

    pub fn decode(&mut self, bus: &Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _ => panic!("Unimplemented opcode: {:#02x}", self.ctx.opcode),
        }
    }

    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }

    fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }
}

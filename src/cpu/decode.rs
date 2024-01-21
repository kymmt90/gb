use crate::{
    cpu::{
        instruction::{Bit, Call, Cp, Dec, Inc, Jr, Ld, Nop, Pop, Push, Ret, Rl},
        operand::{Indirect, Cond, Direct16, Direct8, Imm16, Imm8, Reg16, Reg8, IO8 as _},
        Cpu,
    },
    peripherals::Peripherals,
};

pub trait Decode {
    fn decode(&mut self, bus: &mut Peripherals);
    fn cb_prefixed(&mut self, bus: &mut Peripherals);
    fn cb_decode(&mut self, bus: &mut Peripherals);
}

impl Decode for Cpu {
    fn decode(&mut self, bus: &mut Peripherals) {
        if self.ctx.cb {
            self.cb_prefixed(bus);

            return;
        }

        match self.ctx.opcode {
            0x00 => self.nop(bus),
            0x01 => self.ld16(bus, Reg16::BC, Imm16),
            0x02 => self.ld(bus, Indirect::BC, Reg8::A),
            0x03 => self.inc16(bus, Reg16::BC),
            0x04 => self.inc(bus, Reg8::B),
            0x05 => self.dec(bus, Reg8::B),
            0x06 => self.ld(bus, Reg8::B, Imm8),
            0x08 => self.ld16(bus, Direct16, Reg16::SP),
            0x0a => self.ld(bus, Reg8::A, Indirect::BC),
            0x0b => self.dec16(bus, Reg16::BC),
            0x0c => self.inc(bus, Reg8::C),
            0x0d => self.dec(bus, Reg8::C),
            0x0e => self.ld(bus, Reg8::C, Imm8),
            0x11 => self.ld16(bus, Reg16::DE, Imm16),
            0x12 => self.ld(bus, Indirect::DE, Reg8::A),
            0x13 => self.inc16(bus, Reg16::DE),
            0x14 => self.inc(bus, Reg8::D),
            0x15 => self.dec(bus, Reg8::D),
            0x16 => self.ld(bus, Reg8::D, Imm8),
            0x18 => self.jr(bus),
            0x1a => self.ld(bus, Reg8::A, Indirect::DE),
            0x1b => self.dec16(bus, Reg16::DE),
            0x1c => self.inc(bus, Reg8::E),
            0x1d => self.dec(bus, Reg8::E),
            0x1e => self.ld(bus, Reg8::E, Imm8),
            0x20 => self.jr_c(bus, Cond::NZ),
            0x21 => self.ld16(bus, Reg16::HL, Imm16),
            0x22 => self.ld(bus, Indirect::HLI, Reg8::A),
            0x23 => self.inc16(bus, Reg16::HL),
            0x24 => self.inc(bus, Reg8::H),
            0x25 => self.dec(bus, Reg8::H),
            0x26 => self.ld(bus, Reg8::H, Imm8),
            0x28 => self.jr_c(bus, Cond::Z),
            0x2a => self.ld(bus, Reg8::A, Indirect::HLI),
            0x2b => self.dec16(bus, Reg16::HL),
            0x2c => self.inc(bus, Reg8::L),
            0x2d => self.dec(bus, Reg8::L),
            0x2e => self.ld(bus, Reg8::L, Imm8),
            0x30 => self.jr_c(bus, Cond::NC),
            0x31 => self.ld16(bus, Reg16::SP, Imm16),
            0x32 => self.ld(bus, Indirect::HLD, Reg8::A),
            0x33 => self.inc16(bus, Reg16::SP),
            0x34 => self.inc16(bus, Reg16::HL),
            0x35 => self.dec16(bus, Reg16::HL),
            0x36 => self.ld(bus, Indirect::HL, Imm8),
            0x38 => self.jr_c(bus, Cond::C),
            0x3a => self.ld(bus, Reg8::A, Indirect::HLD),
            0x3b => self.dec16(bus, Reg16::SP),
            0x3c => self.inc(bus, Reg8::A),
            0x3d => self.dec(bus, Reg8::A),
            0x3e => self.ld(bus, Reg8::A, Imm8),
            0x40 => self.ld(bus, Reg8::B, Reg8::B),
            0x41 => self.ld(bus, Reg8::B, Reg8::C),
            0x42 => self.ld(bus, Reg8::B, Reg8::D),
            0x43 => self.ld(bus, Reg8::B, Reg8::E),
            0x44 => self.ld(bus, Reg8::B, Reg8::H),
            0x45 => self.ld(bus, Reg8::B, Reg8::L),
            0x46 => self.ld(bus, Reg8::B, Indirect::HL),
            0x47 => self.ld(bus, Reg8::B, Reg8::A),
            0x48 => self.ld(bus, Reg8::C, Reg8::B),
            0x49 => self.ld(bus, Reg8::C, Reg8::C),
            0x4a => self.ld(bus, Reg8::C, Reg8::D),
            0x4b => self.ld(bus, Reg8::C, Reg8::E),
            0x4c => self.ld(bus, Reg8::C, Reg8::H),
            0x4d => self.ld(bus, Reg8::C, Reg8::L),
            0x4e => self.ld(bus, Reg8::C, Indirect::HL),
            0x50 => self.ld(bus, Reg8::D, Reg8::B),
            0x51 => self.ld(bus, Reg8::D, Reg8::C),
            0x52 => self.ld(bus, Reg8::D, Reg8::D),
            0x53 => self.ld(bus, Reg8::D, Reg8::E),
            0x54 => self.ld(bus, Reg8::D, Reg8::H),
            0x55 => self.ld(bus, Reg8::D, Reg8::L),
            0x56 => self.ld(bus, Reg8::D, Indirect::HL),
            0x57 => self.ld(bus, Reg8::D, Reg8::A),
            0x58 => self.ld(bus, Reg8::E, Reg8::B),
            0x59 => self.ld(bus, Reg8::E, Reg8::C),
            0x5a => self.ld(bus, Reg8::E, Reg8::D),
            0x5b => self.ld(bus, Reg8::E, Reg8::E),
            0x5c => self.ld(bus, Reg8::E, Reg8::H),
            0x5d => self.ld(bus, Reg8::E, Reg8::L),
            0x5e => self.ld(bus, Reg8::E, Indirect::HL),
            0x60 => self.ld(bus, Reg8::H, Reg8::B),
            0x61 => self.ld(bus, Reg8::H, Reg8::C),
            0x62 => self.ld(bus, Reg8::H, Reg8::D),
            0x63 => self.ld(bus, Reg8::H, Reg8::E),
            0x64 => self.ld(bus, Reg8::H, Reg8::H),
            0x65 => self.ld(bus, Reg8::H, Reg8::L),
            0x66 => self.ld(bus, Reg8::H, Indirect::HL),
            0x67 => self.ld(bus, Reg8::H, Reg8::A),
            0x68 => self.ld(bus, Reg8::L, Reg8::B),
            0x69 => self.ld(bus, Reg8::L, Reg8::C),
            0x6a => self.ld(bus, Reg8::L, Reg8::D),
            0x6b => self.ld(bus, Reg8::L, Reg8::E),
            0x6c => self.ld(bus, Reg8::L, Reg8::H),
            0x6d => self.ld(bus, Reg8::L, Reg8::L),
            0x6e => self.ld(bus, Reg8::L, Indirect::HL),
            0x70 => self.ld(bus, Indirect::HL, Reg8::B),
            0x71 => self.ld(bus, Indirect::HL, Reg8::C),
            0x72 => self.ld(bus, Indirect::HL, Reg8::D),
            0x73 => self.ld(bus, Indirect::HL, Reg8::E),
            0x74 => self.ld(bus, Indirect::HL, Reg8::H),
            0x75 => self.ld(bus, Indirect::HL, Reg8::L),
            0x77 => self.ld(bus, Indirect::HL, Reg8::A),
            0x78 => self.ld(bus, Reg8::A, Reg8::B),
            0x79 => self.ld(bus, Reg8::A, Reg8::C),
            0x7a => self.ld(bus, Reg8::A, Reg8::D),
            0x7b => self.ld(bus, Reg8::A, Reg8::E),
            0x7c => self.ld(bus, Reg8::A, Reg8::H),
            0x7d => self.ld(bus, Reg8::A, Reg8::L),
            0x7e => self.ld(bus, Reg8::A, Indirect::HL),
            0xb8 => self.cp(bus, Reg8::B),
            0xb9 => self.cp(bus, Reg8::C),
            0xba => self.cp(bus, Reg8::D),
            0xbb => self.cp(bus, Reg8::E),
            0xbc => self.cp(bus, Reg8::H),
            0xbd => self.cp(bus, Reg8::L),
            0xbe => self.cp(bus, Indirect::HL),
            0xc1 => self.pop(bus, Reg16::BC),
            0xc5 => self.push(bus, Reg16::BC),
            0xc9 => self.ret(bus),
            0xcb => self.cb_prefixed(bus),
            0xcd => self.call(bus),
            0xd1 => self.pop(bus, Reg16::DE),
            0xd5 => self.push(bus, Reg16::DE),
            0xe0 => self.ld(bus, Direct8::DFF, Reg8::A),
            0xe1 => self.pop(bus, Reg16::HL),
            0xe2 => self.ld(bus, Indirect::CFF, Reg8::A),
            0xe5 => self.push(bus, Reg16::HL),
            0xea => self.ld(bus, Direct8::D, Reg8::A),
            0xf0 => self.ld(bus, Reg8::A, Direct8::DFF),
            0xf1 => self.pop(bus, Reg16::AF),
            0xf2 => self.ld(bus, Reg8::A, Indirect::CFF),
            0xf5 => self.push(bus, Reg16::AF),
            0xfa => self.ld(bus, Reg8::A, Direct8::D),
            0xfe => self.cp(bus, Imm8),
            _ => panic!("Unimplemented opcode: {:#02x}", self.ctx.opcode),
        }
    }

    fn cb_prefixed(&mut self, bus: &mut Peripherals) {
        if let Some(v) = self.read8(bus, Imm8) {
            self.ctx.opcode = v;
            self.ctx.cb = true;
            self.cb_decode(bus);
        }
    }

    fn cb_decode(&mut self, bus: &mut Peripherals) {
        match self.ctx.opcode {
            0x10 => self.rl(bus, Reg8::B),
            0x11 => self.rl(bus, Reg8::C),
            0x12 => self.rl(bus, Reg8::D),
            0x13 => self.rl(bus, Reg8::E),
            0x14 => self.rl(bus, Reg8::H),
            0x15 => self.rl(bus, Reg8::L),
            0x16 => self.rl(bus, Indirect::HL),
            0x17 => self.rl(bus, Reg8::A),
            0x40 => self.bit(bus, 0, Reg8::B),
            0x41 => self.bit(bus, 0, Reg8::C),
            0x42 => self.bit(bus, 0, Reg8::D),
            0x43 => self.bit(bus, 0, Reg8::E),
            0x44 => self.bit(bus, 0, Reg8::H),
            0x45 => self.bit(bus, 0, Reg8::L),
            0x46 => self.bit(bus, 0, Indirect::HL),
            0x47 => self.bit(bus, 0, Reg8::A),
            0x48 => self.bit(bus, 1, Reg8::B),
            0x49 => self.bit(bus, 1, Reg8::C),
            0x4a => self.bit(bus, 1, Reg8::D),
            0x4b => self.bit(bus, 1, Reg8::E),
            0x4c => self.bit(bus, 1, Reg8::H),
            0x4d => self.bit(bus, 1, Reg8::L),
            0x4e => self.bit(bus, 1, Indirect::HL),
            0x4f => self.bit(bus, 1, Reg8::A),
            0x50 => self.bit(bus, 2, Reg8::B),
            0x51 => self.bit(bus, 2, Reg8::C),
            0x52 => self.bit(bus, 2, Reg8::D),
            0x53 => self.bit(bus, 2, Reg8::E),
            0x54 => self.bit(bus, 2, Reg8::H),
            0x55 => self.bit(bus, 2, Reg8::L),
            0x56 => self.bit(bus, 2, Indirect::HL),
            0x57 => self.bit(bus, 2, Reg8::A),
            0x58 => self.bit(bus, 3, Reg8::B),
            0x59 => self.bit(bus, 3, Reg8::C),
            0x5a => self.bit(bus, 3, Reg8::D),
            0x5b => self.bit(bus, 3, Reg8::E),
            0x5c => self.bit(bus, 3, Reg8::H),
            0x5d => self.bit(bus, 3, Reg8::L),
            0x5e => self.bit(bus, 3, Indirect::HL),
            0x5f => self.bit(bus, 3, Reg8::A),
            0x60 => self.bit(bus, 4, Reg8::B),
            0x61 => self.bit(bus, 4, Reg8::C),
            0x62 => self.bit(bus, 4, Reg8::D),
            0x63 => self.bit(bus, 4, Reg8::E),
            0x64 => self.bit(bus, 4, Reg8::H),
            0x65 => self.bit(bus, 4, Reg8::L),
            0x66 => self.bit(bus, 4, Indirect::HL),
            0x67 => self.bit(bus, 4, Reg8::A),
            0x68 => self.bit(bus, 5, Reg8::B),
            0x69 => self.bit(bus, 5, Reg8::C),
            0x6a => self.bit(bus, 5, Reg8::D),
            0x6b => self.bit(bus, 5, Reg8::E),
            0x6c => self.bit(bus, 5, Reg8::H),
            0x6d => self.bit(bus, 5, Reg8::L),
            0x6e => self.bit(bus, 5, Indirect::HL),
            0x6f => self.bit(bus, 5, Reg8::A),
            0x70 => self.bit(bus, 6, Reg8::B),
            0x71 => self.bit(bus, 6, Reg8::C),
            0x72 => self.bit(bus, 6, Reg8::D),
            0x73 => self.bit(bus, 6, Reg8::E),
            0x74 => self.bit(bus, 6, Reg8::H),
            0x75 => self.bit(bus, 6, Reg8::L),
            0x76 => self.bit(bus, 6, Indirect::HL),
            0x77 => self.bit(bus, 6, Reg8::A),
            0x78 => self.bit(bus, 7, Reg8::B),
            0x79 => self.bit(bus, 7, Reg8::C),
            0x7a => self.bit(bus, 7, Reg8::D),
            0x7b => self.bit(bus, 7, Reg8::E),
            0x7c => self.bit(bus, 7, Reg8::H),
            0x7d => self.bit(bus, 7, Reg8::L),
            0x7e => self.bit(bus, 7, Indirect::HL),
            0x7f => self.bit(bus, 7, Reg8::A),
            _ => panic!("Unimplemented opcode: {:#02x}", self.ctx.opcode),
        }
    }
}

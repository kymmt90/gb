mod cond;
mod direct16;
mod direct8;
mod imm16;
mod imm8;
mod indirect;
mod reg16;
mod reg8;

pub use cond::Cond;
pub use imm16::Imm16;
pub use imm8::Imm8;
pub use reg16::Reg16;

use crate::peripherals::Peripherals;

pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8) -> Option<()>;
}

pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16) -> Option<()>;
}

macro_rules! step {
    ($d:expr, {$($c:tt : $e:expr,)*}) => {
        static STEP: AtomicU8 = AtomicU8::new(0);
        #[allow(dead_code)]
        static VAL8: AtomicU8 = AtomicU8::new(0);
        #[allow(dead_code)]
        static VAL16: AtomicU16 = AtomicU16::new(0);
        $(if STEP.load(Relaxed) == $c { $e })* else {
            #[allow(clippy::unused_unit)]
            return $d;
        }
    };
}

macro_rules! go {
    ($e:expr) => {
        STEP.store($e, Relaxed);
    };
}

pub(crate) use go;
pub(crate) use step;

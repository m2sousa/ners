#[rustfmt::skip]
#[allow(non_snake_case)]
pub mod StatusFlags {
    pub const CARRY: u8                 = 0x1;
    pub const ZERO: u8                  = 0x2;
    pub const INTERRUPT_DISABLE: u8     = 0x4;
    pub const DECIMAL: u8               = 0x8;
    pub const B: u8                     = 0x10;
    // Bit at 0x20 is not used, and always set to 1.
    pub const UNUSED: u8                = 0x20;
    pub const OVERFLOW: u8              = 0x40;
    pub const NEGATIVE: u8              = 0x80;
}

type FlagPosition = u8;

#[derive(Default)]
pub(crate) struct Registers {
    pub pc: u16,    // Program Counter
    pub sp: u8,     // Stack Pointer
    pub acc: u8,    // Accumulator
    pub x: u8,      // Index Register X
    pub y: u8,      // Index Register Y
    pub status: u8, // Processor status
}

impl Registers {
    pub fn init() -> Self {
        Registers {
            // Program ROM -- PRG ROM [0x8000 ... 0xFFFF]
            // FIXME: program_counter must be read from 0xfffc as well ?
            pc: 0x8000,
            // Memory space [0x0100 ... 0x01FF] is used for stack. Stack grows from top to bottom.
            // Why does the stack pointer starts at 0xfd rather than 0xff tho ?
            sp: 0xfd,
            // Status bit at 0x20 (fifth bit) is always set to 1.
            status: StatusFlags::UNUSED | StatusFlags::INTERRUPT_DISABLE,
            ..Default::default()
        }
    }

    /// Reset the registers to their initial value. Note that the program_counter must be read
    /// behorehand from 0xfffc in memory and thus passed to this method.
    pub fn reset(&mut self, pc: u16) {
        self.pc = pc;
        self.sp = self.sp.wrapping_sub(3);
        self.set(StatusFlags::INTERRUPT_DISABLE);
    }

    pub fn set(&mut self, flag: FlagPosition) {
        self.status |= flag;
    }

    pub fn unset(&mut self, flag: FlagPosition) {
        self.status &= !flag;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_flags() {
        let mut reg = Registers::init();
        reg.set(StatusFlags::NEGATIVE);
        assert_eq!(
            reg.status,
            StatusFlags::UNUSED | StatusFlags::INTERRUPT_DISABLE | StatusFlags::NEGATIVE
        );
    }

    #[test]
    fn test_unset_flags() {
        let mut reg = Registers::init();
        reg.set(StatusFlags::NEGATIVE);
        assert_eq!(
            reg.status,
            StatusFlags::UNUSED | StatusFlags::INTERRUPT_DISABLE | StatusFlags::NEGATIVE
        );
        reg.unset(StatusFlags::NEGATIVE);
        assert_eq!(
            reg.status,
            StatusFlags::UNUSED | StatusFlags::INTERRUPT_DISABLE
        );
    }
}

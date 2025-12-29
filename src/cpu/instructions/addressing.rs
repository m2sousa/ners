use super::Cpu;

#[derive(Clone, Copy)]
pub(crate) enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX, // Indexed Indirect
    IndirectY, // Indirect Indexed
}

impl Cpu {
    pub(super) fn read_operand(&self, mode: AddressingMode) -> u8 {
        let operand: u8 = match mode {
            AddressingMode::Immediate => self.mem_read(self.reg.pc + 1),
            AddressingMode::ZeroPage => {
                // Due to the cast as u16, this will address the first page naturally
                let lo = self.mem_read(self.reg.pc + 1) as u16;
                self.mem_read(lo)
            }
            AddressingMode::Absolute => {
                let lo = self.mem_read(self.reg.pc + 1) as u16;
                let hi = self.mem_read(self.reg.pc + 2) as u16;

                let addr = (hi << 8) | lo;
                self.mem_read(addr)
            }
            _ => unimplemented!("AddressingMode read_operand not implemented."),
        };

        operand
    }
}

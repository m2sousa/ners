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
            AddressingMode::ZeroPageX => {
                let lo = self.mem_read(self.reg.pc + 1);
                let addr = lo.wrapping_add(self.reg.x) as u16;
                self.mem_read(addr)
            }
            AddressingMode::ZeroPageY => {
                // TODO: must be tested with ldx for example
                let lo = self.mem_read(self.reg.pc + 1);
                let addr = lo.wrapping_add(self.reg.y) as u16;
                self.mem_read(addr)
            }
            AddressingMode::Absolute => {
                let lo = self.mem_read(self.reg.pc + 1) as u16;
                let hi = self.mem_read(self.reg.pc + 2) as u16;

                let addr = (hi << 8) | lo;
                self.mem_read(addr)
            }
            AddressingMode::AbsoluteX => {
                // TODO: must be tested with lda for example
                let lo = self.mem_read(self.reg.pc + 1) as u16;
                let hi = self.mem_read(self.reg.pc + 2) as u16;

                let addr = ((hi << 8) | lo).wrapping_add(self.reg.x as u16);
                self.mem_read(addr)
            }
            AddressingMode::AbsoluteY => {
                // TODO: must be tested with lda for example
                let lo = self.mem_read(self.reg.pc + 1) as u16;
                let hi = self.mem_read(self.reg.pc + 2) as u16;

                let addr = ((hi << 8) | lo).wrapping_add(self.reg.y as u16);
                self.mem_read(addr)
            }
            AddressingMode::Indirect => {
                // TODO: must be tested with jmp (only instruction that supports indirection)
                let plo = self.mem_read(self.reg.pc + 1) as u16;
                let phi = self.mem_read(self.reg.pc + 2) as u16;

                let paddr = (phi << 8) | plo;

                let lo = self.mem_read(paddr) as u16;
                let hi = self.mem_read(paddr + 1) as u16;

                let addr = (hi << 8) | lo;
                self.mem_read(addr)
            }
            AddressingMode::IndirectX => {
                // TODO: must be tested with lda for example
                let lo = self.mem_read(self.reg.pc + 1);

                let paddr = lo.wrapping_add(self.reg.x) as u16;

                let lo = self.mem_read(paddr) as u16;
                let hi = self.mem_read(paddr + 1) as u16;

                let addr = (hi << 8) | lo;
                self.mem_read(addr)
            }
            AddressingMode::IndirectY => {
                let paddr = self.mem_read(self.reg.pc + 1) as u16;

                let lo = self.mem_read(paddr) as u16;
                let hi = self.mem_read(paddr + 1) as u16;

                let addr = ((hi << 8) | lo).wrapping_add(self.reg.y as u16);
                self.mem_read(addr)
            }
            _ => unimplemented!("AddressingMode read_operand not implemented."),
        };

        operand
    }
}

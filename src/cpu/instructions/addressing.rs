use super::Cpu;

#[derive(Clone, Copy, Debug)]
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
    pub(super) fn get_operand_address(&self, mode: AddressingMode) -> u16 {
        match mode {
            // Note that the Relative addressing mode returns the same value as would the Immediate
            // one, this implies that instructions are responsible to the u8 to i8 conversion
            // before applying its operation(s).
            AddressingMode::Immediate | AddressingMode::Relative => self.reg.pc + 1,
            AddressingMode::ZeroPage => self.mem_read(self.reg.pc + 1) as u16,
            AddressingMode::ZeroPageX => {
                let lo = self.mem_read(self.reg.pc + 1);

                lo.wrapping_add(self.reg.x) as u16
            }
            AddressingMode::ZeroPageY => {
                let lo = self.mem_read(self.reg.pc + 1);

                lo.wrapping_add(self.reg.y) as u16
            }
            AddressingMode::Absolute => {
                let lo = self.mem_read(self.reg.pc + 1) as u16;
                let hi = self.mem_read(self.reg.pc + 2) as u16;

                (hi << 8) | lo
            }
            AddressingMode::AbsoluteX => {
                let lo = self.mem_read(self.reg.pc + 1) as u16;
                let hi = self.mem_read(self.reg.pc + 2) as u16;

                ((hi << 8) | lo).wrapping_add(self.reg.x as u16)
            }
            AddressingMode::AbsoluteY => {
                let lo = self.mem_read(self.reg.pc + 1) as u16;
                let hi = self.mem_read(self.reg.pc + 2) as u16;

                ((hi << 8) | lo).wrapping_add(self.reg.y as u16)
            }
            AddressingMode::Indirect => {
                let plo = self.mem_read(self.reg.pc + 1) as u16;
                let phi = self.mem_read(self.reg.pc + 2) as u16;

                let paddr = (phi << 8) | plo;

                let lo = self.mem_read(paddr) as u16;
                let hi = self.mem_read(paddr + 1) as u16;

                (hi << 8) | lo
            }
            AddressingMode::IndirectX => {
                let lo = self.mem_read(self.reg.pc + 1);

                let paddr = lo.wrapping_add(self.reg.x) as u16;

                let lo = self.mem_read(paddr) as u16;
                let hi = self.mem_read((paddr + 1) & 0xff) as u16;

                (hi << 8) | lo
            }
            AddressingMode::IndirectY => {
                let paddr = self.mem_read(self.reg.pc + 1) as u16;

                let lo = self.mem_read(paddr) as u16;
                let hi = self.mem_read((paddr + 1) & 0xff) as u16;

                ((hi << 8) | lo).wrapping_add(self.reg.y as u16)
            }
            _ => unimplemented!("Method `get_operand_address` not implemented for {mode:?}.",),
        }
    }

    /// Due to a cpu bug, a peculiar handling must be done for the JMP instruction with the
    /// addressing mode being indirect. The cpu failing to increment the page when addresses end
    /// with 0xff, thus, only lsb must be incremented in this case.
    pub(super) fn get_jmp_operand_address(&self, mode: AddressingMode) -> u16 {
        if !matches!(mode, AddressingMode::Indirect) {
            return self.get_operand_address(mode);
        }

        let plo = self.mem_read(self.reg.pc + 1) as u16;
        let phi = self.mem_read(self.reg.pc + 2) as u16;

        let lo = self.mem_read((phi << 8) | plo) as u16;
        let hi = self.mem_read((phi << 8) | (plo.wrapping_add(1) & 0xff)) as u16;

        (hi << 8) | lo
    }

    pub(super) fn get_instruction_operand(&self, mode: AddressingMode) -> u8 {
        // Some instructions have an option to operate directly upon the accumulator.
        // This early return permit a smooth use of those instructions.
        if matches!(mode, AddressingMode::Accumulator) {
            return self.reg.acc;
        }

        let addr = self.get_operand_address(mode);

        self.mem_read(addr)
    }
}

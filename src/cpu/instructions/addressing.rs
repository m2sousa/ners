use super::Cpu;

#[derive(Clone, Copy)]
pub(crate) enum AddressingMode {
    Implied,
    Immediate,
}

impl Cpu {
    pub(crate) fn read_operand(&self, mode: AddressingMode) -> u8 {
        let operand: u8 = match mode {
            AddressingMode::Immediate => self.mem[(self.reg.pc + 1) as usize],
            _ => unimplemented!("AddressingMode read_operand not implemented."),
        };

        operand
    }
}

use super::Cpu;

pub(crate) enum AddressingModes {
    Immediate,
    Implied,
}

pub struct Instruction(
    &'static str,                              // Name
    fn(&mut Cpu, &AddressingModes, u16) -> (), // Associated method
    AddressingModes,                           // Addressing mode
    u16,                                       // Length
    &'static str, // Timing (must I save a boolean for paging or a &str is fine?)
);

impl Instruction {
    pub fn exec(&self, cpu: &mut Cpu) {
        let executor = self.1;
        let addr_mode = &self.2;
        let length = self.3;
        executor(cpu, addr_mode, length);
    }
}

#[allow(non_snake_case)]
impl Cpu {
    #[rustfmt::skip]
    pub(crate) const INSTRUCTIONS: &[(u8, Instruction)] = &[
        (0x00, Instruction("BRK", Self::BRK, AddressingModes::Implied,      1, "7")),
        (0xA9, Instruction("LDA", Self::LDA, AddressingModes::Immediate,    2, "2")),
        (0xAA, Instruction("TAX", Self::TAX, AddressingModes::Implied,      1, "2")),
        (0xE8, Instruction("INX", Self::INX, AddressingModes::Implied,      1, "2")),
    ];

    pub(crate) fn get_instruction_from_opcode(opcode: u8) -> &'static Instruction {
        let instruction = &Self::INSTRUCTIONS
            .iter()
            .find(|&I| I.0 == opcode)
            .unwrap() // FIXME: should I unwrap() here ? Probably not ?
            .1;

        instruction
    }

    fn BRK(&mut self, _: &AddressingModes, _: u16) {
        self.is_running = false;
    }

    fn LDA(&mut self, addr_mode: &AddressingModes, length: u16) {
        let result: u8 = match *addr_mode {
            AddressingModes::Immediate => self.mem[self.reg.pc as usize],
            _ => todo!("LDA addressing modes must be implemented."),
        };

        self.reg.pc += length - 1;

        self.reg.acc = result;

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    fn TAX(&mut self, _: &AddressingModes, _: u16) {
        self.reg.x = self.reg.acc;

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }

    fn INX(&mut self, _: &AddressingModes, _: u16) {
        self.reg.x += 1;

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }
}

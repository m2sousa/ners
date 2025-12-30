mod addressing;
mod ops;

use super::Cpu;
use addressing::AddressingMode;

pub(crate) struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub execute: fn(&mut Cpu, AddressingMode) -> (),
    pub mode: AddressingMode,
    pub length: u8,
    pub cycles: Cycles,
}

pub(crate) struct Cycles {
    pub base: u8,
    pub page_boundary_penalty: bool,
}

impl Instruction {
    const fn new(
        opcode: u8,
        mnemonic: &'static str,
        execute: fn(&mut Cpu, AddressingMode) -> (),
        mode: AddressingMode,
        length: u8,
        cycles: u8,
        page_penalty: bool,
    ) -> Self {
        Self {
            opcode,
            mnemonic,
            execute,
            mode,
            length,
            cycles: Cycles {
                base: cycles,
                page_boundary_penalty: page_penalty,
            },
        }
    }
}

impl Cpu {
    #[rustfmt::skip]
    pub(super) const INSTRUCTION_SET: &'static [Instruction] = &[
        Instruction::new(0x00, "BRK", Cpu::BRK, AddressingMode::Implied,		1, 7, false),
        Instruction::new(0x01, "ORA", Cpu::ORA, AddressingMode::IndirectX,		2, 6, false),
        Instruction::new(0x05, "ORA", Cpu::ORA, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0x06, "ASL", Cpu::ASL, AddressingMode::ZeroPage,		2, 5, false),
        Instruction::new(0x08, "PHP", Cpu::PHP, AddressingMode::Implied,		1, 3, false),
        Instruction::new(0x09, "ORA", Cpu::ORA, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0x0a, "ASL", Cpu::ASL, AddressingMode::Accumulator,	1, 2, false),
        Instruction::new(0x0d, "ORA", Cpu::ORA, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0x0e, "ASL", Cpu::ASL, AddressingMode::Absolute,		3, 6, false),
        Instruction::new(0x10, "BPL", Cpu::BPL, AddressingMode::Relative,		2, 2, false),
        Instruction::new(0x11, "ORA", Cpu::ORA, AddressingMode::IndirectY,		2, 5, true),
        Instruction::new(0x15, "ORA", Cpu::ORA, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0x16, "ASL", Cpu::ASL, AddressingMode::ZeroPageX,		2, 6, false),
        Instruction::new(0x18, "CLC", Cpu::CLC, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0x19, "ORA", Cpu::ORA, AddressingMode::AbsoluteY,		3, 4, true),
        Instruction::new(0x1d, "ORA", Cpu::ORA, AddressingMode::AbsoluteX,		3, 4, true),
        Instruction::new(0x1e, "ASL", Cpu::ASL, AddressingMode::AbsoluteX,		3, 7, false),
        Instruction::new(0x20, "JSR", Cpu::JSR, AddressingMode::Absolute,		3, 6, false),
        Instruction::new(0x21, "AND", Cpu::AND, AddressingMode::IndirectX,		2, 6, false),
        Instruction::new(0x24, "BIT", Cpu::BIT, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0x25, "AND", Cpu::AND, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0x26, "ROL", Cpu::ROL, AddressingMode::ZeroPage,		2, 5, false),
        Instruction::new(0x28, "PLP", Cpu::PLP, AddressingMode::Implied,		1, 4, false),
        Instruction::new(0x29, "AND", Cpu::AND, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0x2a, "ROL", Cpu::ROL, AddressingMode::Accumulator,	1, 2, false),
        Instruction::new(0x2c, "BIT", Cpu::BIT, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0x2d, "AND", Cpu::AND, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0x2e, "ROL", Cpu::ROL, AddressingMode::Absolute,		3, 6, false),
        Instruction::new(0x30, "BMI", Cpu::BMI, AddressingMode::Relative,		2, 2, false),
        Instruction::new(0x31, "AND", Cpu::AND, AddressingMode::IndirectY,		2, 5, true),
        Instruction::new(0x35, "AND", Cpu::AND, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0x36, "ROL", Cpu::ROL, AddressingMode::ZeroPageX,		2, 6, false),
        Instruction::new(0x38, "SEC", Cpu::SEC, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0x39, "AND", Cpu::AND, AddressingMode::AbsoluteY,		3, 4, true),
        Instruction::new(0x3d, "AND", Cpu::AND, AddressingMode::AbsoluteX,		3, 4, true),
        Instruction::new(0x3e, "ROL", Cpu::ROL, AddressingMode::AbsoluteX,		3, 7, false),
        Instruction::new(0x40, "RTI", Cpu::RTI, AddressingMode::Implied,		1, 6, false),
        Instruction::new(0x41, "EOR", Cpu::EOR, AddressingMode::IndirectX,		2, 6, false),
        Instruction::new(0x45, "EOR", Cpu::EOR, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0x46, "LSR", Cpu::LSR, AddressingMode::ZeroPage,		2, 5, false),
        Instruction::new(0x48, "PHA", Cpu::PHA, AddressingMode::Implied,		1, 3, false),
        Instruction::new(0x49, "EOR", Cpu::EOR, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0x4a, "LSR", Cpu::LSR, AddressingMode::Accumulator,	1, 2, false),
        Instruction::new(0x4c, "JMP", Cpu::JMP, AddressingMode::Absolute,		3, 3, false),
        Instruction::new(0x4d, "EOR", Cpu::EOR, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0x4e, "LSR", Cpu::LSR, AddressingMode::Absolute,		3, 6, false),
        Instruction::new(0x50, "BVC", Cpu::BVC, AddressingMode::Relative,		2, 2, false),
        Instruction::new(0x51, "EOR", Cpu::EOR, AddressingMode::IndirectY,		2, 5, true),
        Instruction::new(0x55, "EOR", Cpu::EOR, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0x56, "LSR", Cpu::LSR, AddressingMode::ZeroPageX,		2, 6, false),
        Instruction::new(0x58, "CLI", Cpu::CLI, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0x59, "EOR", Cpu::EOR, AddressingMode::AbsoluteY,		3, 4, true),
        Instruction::new(0x5d, "EOR", Cpu::EOR, AddressingMode::AbsoluteX,		3, 4, true),
        Instruction::new(0x5e, "LSR", Cpu::LSR, AddressingMode::AbsoluteX,		3, 7, false),
        Instruction::new(0x60, "RTS", Cpu::RTS, AddressingMode::Implied,		1, 6, false),
        Instruction::new(0x61, "ADC", Cpu::ADC, AddressingMode::IndirectX,		2, 6, false),
        Instruction::new(0x65, "ADC", Cpu::ADC, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0x66, "ROR", Cpu::ROR, AddressingMode::ZeroPage,		2, 5, false),
        Instruction::new(0x68, "PLA", Cpu::PLA, AddressingMode::Implied,		1, 4, false),
        Instruction::new(0x69, "ADC", Cpu::ADC, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0x6a, "ROR", Cpu::ROR, AddressingMode::Accumulator,	1, 2, false),
        Instruction::new(0x6c, "JMP", Cpu::JMP, AddressingMode::Indirect,		3, 5, false),
        Instruction::new(0x6d, "ADC", Cpu::ADC, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0x6e, "ROR", Cpu::ROR, AddressingMode::Absolute,		3, 6, false),
        Instruction::new(0x70, "BVS", Cpu::BVS, AddressingMode::Relative,		2, 2, false),
        Instruction::new(0x71, "ADC", Cpu::ADC, AddressingMode::IndirectY,		2, 5, true),
        Instruction::new(0x75, "ADC", Cpu::ADC, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0x76, "ROR", Cpu::ROR, AddressingMode::ZeroPageX,		2, 6, false),
        Instruction::new(0x78, "SEI", Cpu::SEI, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0x79, "ADC", Cpu::ADC, AddressingMode::AbsoluteY,		3, 4, true),
        Instruction::new(0x7d, "ADC", Cpu::ADC, AddressingMode::AbsoluteX,		3, 4, true),
        Instruction::new(0x7e, "ROR", Cpu::ROR, AddressingMode::AbsoluteX,		3, 7, false),
        Instruction::new(0x81, "STA", Cpu::STA, AddressingMode::IndirectX,		2, 6, false),
        Instruction::new(0x84, "STY", Cpu::STY, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0x85, "STA", Cpu::STA, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0x86, "STX", Cpu::STX, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0x88, "DEY", Cpu::DEY, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0x8a, "TXA", Cpu::TXA, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0x8c, "STY", Cpu::STY, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0x8d, "STA", Cpu::STA, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0x8e, "STX", Cpu::STX, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0x90, "BCC", Cpu::BCC, AddressingMode::Relative,		2, 2, false),
        Instruction::new(0x91, "STA", Cpu::STA, AddressingMode::IndirectY,		2, 6, false),
        Instruction::new(0x94, "STY", Cpu::STY, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0x95, "STA", Cpu::STA, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0x96, "STX", Cpu::STX, AddressingMode::ZeroPageY,		2, 4, false),
        Instruction::new(0x98, "TYA", Cpu::TYA, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0x99, "STA", Cpu::STA, AddressingMode::AbsoluteY,		3, 5, false),
        Instruction::new(0x9a, "TXS", Cpu::TXS, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0x9d, "STA", Cpu::STA, AddressingMode::AbsoluteX,		3, 5, false),
        Instruction::new(0xa0, "LDY", Cpu::LDY, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0xa1, "LDA", Cpu::LDA, AddressingMode::IndirectX,		2, 6, false),
        Instruction::new(0xa2, "LDX", Cpu::LDX, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0xa4, "LDY", Cpu::LDY, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0xa5, "LDA", Cpu::LDA, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0xa6, "LDX", Cpu::LDX, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0xa8, "TAY", Cpu::TAY, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xa9, "LDA", Cpu::LDA, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0xaa, "TAX", Cpu::TAX, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xac, "LDY", Cpu::LDY, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0xad, "LDA", Cpu::LDA, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0xae, "LDX", Cpu::LDX, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0xb0, "BCS", Cpu::BCS, AddressingMode::Relative,		2, 2, false),
        Instruction::new(0xb1, "LDA", Cpu::LDA, AddressingMode::IndirectY,		2, 5, true),
        Instruction::new(0xb4, "LDY", Cpu::LDY, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0xb5, "LDA", Cpu::LDA, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0xb6, "LDX", Cpu::LDX, AddressingMode::ZeroPageY,		2, 4, false),
        Instruction::new(0xb8, "CLV", Cpu::CLV, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xb9, "LDA", Cpu::LDA, AddressingMode::AbsoluteY,		3, 4, true),
        Instruction::new(0xba, "TSX", Cpu::TSX, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xbc, "LDY", Cpu::LDY, AddressingMode::AbsoluteX,		3, 4, true),
        Instruction::new(0xbd, "LDA", Cpu::LDA, AddressingMode::AbsoluteX,		3, 4, true),
        Instruction::new(0xbe, "LDX", Cpu::LDX, AddressingMode::AbsoluteY,		3, 4, true),
        Instruction::new(0xc0, "CPY", Cpu::CPY, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0xc1, "CMP", Cpu::CMP, AddressingMode::IndirectX,		2, 6, false),
        Instruction::new(0xc4, "CPY", Cpu::CPY, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0xc5, "CMP", Cpu::CMP, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0xc6, "DEC", Cpu::DEC, AddressingMode::ZeroPage,		2, 5, false),
        Instruction::new(0xc8, "INY", Cpu::INY, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xc9, "CMP", Cpu::CMP, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0xca, "DEX", Cpu::DEX, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xcc, "CPY", Cpu::CPY, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0xcd, "CMP", Cpu::CMP, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0xce, "DEC", Cpu::DEC, AddressingMode::Absolute,		3, 6, false),
        Instruction::new(0xd0, "BNE", Cpu::BNE, AddressingMode::Relative,		2, 2, false),
        Instruction::new(0xd1, "CMP", Cpu::CMP, AddressingMode::IndirectY,		2, 5, true),
        Instruction::new(0xd5, "CMP", Cpu::CMP, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0xd6, "DEC", Cpu::DEC, AddressingMode::ZeroPageX,		2, 6, false),
        Instruction::new(0xd8, "CLD", Cpu::CLD, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xd9, "CMP", Cpu::CMP, AddressingMode::AbsoluteY,		3, 4, true),
        Instruction::new(0xdd, "CMP", Cpu::CMP, AddressingMode::AbsoluteX,		3, 4, true),
        Instruction::new(0xde, "DEC", Cpu::DEC, AddressingMode::AbsoluteX,		3, 7, false),
        Instruction::new(0xe0, "CPX", Cpu::CPX, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0xe1, "SBC", Cpu::SBC, AddressingMode::IndirectX,		2, 6, false),
        Instruction::new(0xe4, "CPX", Cpu::CPX, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0xe5, "SBC", Cpu::SBC, AddressingMode::ZeroPage,		2, 3, false),
        Instruction::new(0xe6, "INC", Cpu::INC, AddressingMode::ZeroPage,		2, 5, false),
        Instruction::new(0xe8, "INX", Cpu::INX, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xe9, "SBC", Cpu::SBC, AddressingMode::Immediate,		2, 2, false),
        Instruction::new(0xea, "NOP", Cpu::NOP, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xec, "CPX", Cpu::CPX, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0xed, "SBC", Cpu::SBC, AddressingMode::Absolute,		3, 4, false),
        Instruction::new(0xee, "INC", Cpu::INC, AddressingMode::Absolute,		3, 6, false),
        Instruction::new(0xf0, "BEQ", Cpu::BEQ, AddressingMode::Relative,		2, 2, false),
        Instruction::new(0xf1, "SBC", Cpu::SBC, AddressingMode::IndirectY,		2, 5, true),
        Instruction::new(0xf5, "SBC", Cpu::SBC, AddressingMode::ZeroPageX,		2, 4, false),
        Instruction::new(0xf6, "INC", Cpu::INC, AddressingMode::ZeroPageX,		2, 6, false),
        Instruction::new(0xf8, "SED", Cpu::SED, AddressingMode::Implied,		1, 2, false),
        Instruction::new(0xf9, "SBC", Cpu::SBC, AddressingMode::AbsoluteY,		3, 4, true),
        Instruction::new(0xfd, "SBC", Cpu::SBC, AddressingMode::AbsoluteX,		3, 4, true),
        Instruction::new(0xfe, "INC", Cpu::INC, AddressingMode::AbsoluteX,		3, 7, false),
    ];

    /// Build a 256-entry lookup table for O(1) opcode lookup.
    pub(super) fn build_opcode_table() -> [Option<&'static Instruction>; u8::MAX as usize + 1] {
        let mut opcode_table = [None; u8::MAX as usize + 1];

        for inst in Self::INSTRUCTION_SET {
            opcode_table[inst.opcode as usize] = Some(inst);
        }

        opcode_table
    }

    /// Execute an instruction.
    /// Returns the number of cycles used by the instruction.
    pub(super) fn execute_instruction(&mut self, inst: &Instruction) -> u8 {
        (inst.execute)(self, inst.mode);

        let mut cycles = inst.cycles.base;

        // TODO: must handle page crossing, cycles += 1;

        self.reg.pc += inst.length as u16;

        cycles
    }
}

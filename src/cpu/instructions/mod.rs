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
    pub(crate) const INSTRUCTION_SET: &'static [Instruction] = &[
        Instruction::new(0x00, "BRK", Cpu::BRK, AddressingMode::Implied,   1, 7, false),
        Instruction::new(0xA9, "LDA", Cpu::LDA, AddressingMode::Immediate, 2, 2, false),
        Instruction::new(0xAA, "TAX", Cpu::TAX, AddressingMode::Implied,   1, 2, false),
        Instruction::new(0xE8, "INX", Cpu::INX, AddressingMode::Implied,   1, 2, false),
    ];

    /// Build a 256-entry lookup table for O(1) opcode lookup.
    pub(super) fn build_opcode_table() -> [Option<&'static Instruction>; 256] {
        let mut opcode_table = [None; 256];

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

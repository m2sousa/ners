mod instructions;
mod registers;

use instructions::Instruction;
use registers::{Registers, StatusFlags};

use crate::bus::Bus;

pub struct Cpu {
    reg: Registers,
    is_running: bool,

    opcode_table: [Option<&'static Instruction>; u8::MAX as usize + 1],
}

impl Cpu {
    const PRG_ROM_START: usize = 0x8000;

    pub fn new() -> Self {
        let reg = Registers::init();
        let opcode_table = Self::build_opcode_table();
        Cpu {
            reg,
            is_running: false,
            opcode_table,
        }
    }

    pub fn run(&mut self, bus: &mut Bus) {
        self.is_running = true;
        while self.is_running {
            let op = bus.read(self.reg.pc);

            let cycles_used = match self.opcode_table[op as usize] {
                Some(inst) => self.execute_instruction(bus, inst),
                None => todo!(
                    "instruction (opcode={}) not found, error handling must be implemented",
                    op
                ),
            };
        }
    }

    fn mem_read(&self, bus: &Bus, addr: u16) -> u8 {
        bus.read(addr)
    }

    fn mem_write(&self, bus: &mut Bus, addr: u16, data: u8) {
        bus.write(addr, data);
    }

    fn push_stack(&mut self, bus: &mut Bus, data: u8) {
        // [0x0100 ... 0x01ff] is used for stack data.
        const STACK_BASE: u16 = 0x0100;

        let addr = STACK_BASE + self.reg.sp as u16;

        self.mem_write(bus, addr, data);

        self.reg.sp -= 1;
    }

    fn pull_stack(&mut self, bus: &Bus) -> u8 {
        // [0x0100 ... 0x01ff] is used for stack data.
        const STACK_BASE: u16 = 0x0100;

        self.reg.sp += 1;

        let addr = STACK_BASE + self.reg.sp as u16;

        self.mem_read(bus, addr)
    }

    // TODO: Could I write a method that given a value and flags as StatusFlag::CARRY |
    // StatusFlag::OVERFLOW and so on, do all the work in one go ?
    fn set_carry(&mut self, value: u8) {
        if value != 0 {
            self.reg.set(StatusFlags::CARRY);
        } else {
            self.reg.unset(StatusFlags::CARRY);
        }
    }

    fn set_zero(&mut self, value: u8) {
        if value == 0 {
            self.reg.set(StatusFlags::ZERO);
        } else {
            self.reg.unset(StatusFlags::ZERO);
        }
    }

    fn set_negative(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.reg.set(StatusFlags::NEGATIVE);
        } else {
            self.reg.unset(StatusFlags::NEGATIVE);
        }
    }
}

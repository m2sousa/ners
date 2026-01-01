mod instructions;
mod registers;

use instructions::Instruction;
use registers::{Registers, StatusFlags};

pub struct Cpu {
    reg: Registers,
    mem: [u8; u16::MAX as usize],
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
            mem: [0; u16::MAX as usize],
            is_running: false,
            opcode_table,
        }
    }

    pub fn load(&mut self, prg: Vec<u8>) {
        // TODO: Must check that prg.len() is less or equal than 0xFFFF - 0x8000 (PRG-ROM length in
        // the memory map), but what to do then ?
        self.mem[Self::PRG_ROM_START..Self::PRG_ROM_START + prg.len()].copy_from_slice(&prg);
    }

    pub fn run(&mut self) {
        self.is_running = true;
        while self.is_running {
            let op = self.mem_read(self.reg.pc);

            let cycles_used = match self.opcode_table[op as usize] {
                Some(inst) => self.execute_instruction(inst),
                None => todo!(
                    "instruction (opcode={}) not found, error handling must be implemented",
                    op
                ),
            };
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.mem[addr as usize] = data;
    }

    fn push_stack(&mut self, data: u8) {
        // [0x0100 ... 0x01ff] is used for stack data.
        const STACK_BASE: u16 = 0x0100;

        let addr = STACK_BASE + self.reg.sp as u16;

        self.mem_write(addr, data);

        self.reg.sp -= 1;
    }

    fn pull_stack(&mut self) -> u8 {
        // [0x0100 ... 0x01ff] is used for stack data.
        const STACK_BASE: u16 = 0x0100;

        let addr = STACK_BASE + self.reg.sp as u16;

        self.reg.sp += 1;

        self.mem_read(addr)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_immediate_basic_lda() {
        let mut cpu = Cpu::new();
        let prg: Vec<u8> = vec![0xa9, 0x06, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0x06);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED);
    }

    #[test]
    fn test_zeropage_lda() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x00fa, 0x06);
        let prg: Vec<u8> = vec![0xa5, 0xfa, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0x06);
    }

    #[test]
    fn test_zeropage_x_lda() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x008f, 0x06);
        cpu.reg.x = 0x0f;
        let prg: Vec<u8> = vec![0xb5, 0x80, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0x06);
    }

    #[test]
    fn test_absolute_lda() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x00f1, 0x06);
        let prg: Vec<u8> = vec![0xad, 0xf1, 0x00, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0x06);
    }

    #[test]
    fn test_immediate_zero_lda() {
        let mut cpu = Cpu::new();
        let prg: Vec<u8> = vec![0xa9, 0x00, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0x00);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED | StatusFlags::ZERO);
    }

    #[test]
    fn test_immediate_negative_lda() {
        let mut cpu = Cpu::new();
        let prg: Vec<u8> = vec![0xa9, 0xff, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0xff);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED | StatusFlags::NEGATIVE);
    }

    #[test]
    fn test_tax() {
        let mut cpu = Cpu::new();
        let prg: Vec<u8> = vec![0xa9, 0x0a, 0xaa, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, cpu.reg.x);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED);
    }

    #[test]
    fn test_negative_tax() {
        let mut cpu = Cpu::new();
        let prg: Vec<u8> = vec![0xa9, 0xff, 0xaa, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, cpu.reg.x);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED | StatusFlags::NEGATIVE);
    }

    #[test]
    fn test_zero_tax() {
        let mut cpu = Cpu::new();
        let prg: Vec<u8> = vec![0xa9, 0x00, 0xaa, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, cpu.reg.x);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED | StatusFlags::ZERO);
    }

    #[test]
    fn test_inx() {
        let mut cpu = Cpu::new();
        let prg: Vec<u8> = vec![0xe8, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.x, 1);
    }

    #[test]
    fn test_inx_overflow_and_zero() {
        let mut cpu = Cpu::new();
        cpu.reg.x = 0xff;
        let prg: Vec<u8> = vec![0xe8, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.x, 0);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED | StatusFlags::ZERO);
    }

    #[test]
    fn test_inx_negative() {
        let mut cpu = Cpu::new();
        cpu.reg.x = 0b0111_1111;
        let prg: Vec<u8> = vec![0xe8, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.x, 0b1000_0000);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED | StatusFlags::NEGATIVE);
    }

    #[test]
    fn test_immediate_and() {
        let mut cpu = Cpu::new();
        cpu.reg.acc = 0xff;
        let prg: Vec<u8> = vec![0x29, 0x0e, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0xff & 0x0e);
    }

    #[test]
    fn test_accumulator_asl() {
        let mut cpu = Cpu::new();
        cpu.reg.acc = 0b1000_0001;
        let prg: Vec<u8> = vec![0x0a, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0b0000_0010);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED | StatusFlags::CARRY);
    }

    #[test]
    fn test_absolute_asl() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x0a0b, 0b0000_0001);
        let prg: Vec<u8> = vec![0x0e, 0x0b, 0x0a, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.mem_read(0x0a0b), 0b0000_0010);
        assert_eq!(cpu.reg.status, StatusFlags::UNUSED);
    }

    #[test]
    fn test_zeropage_inc() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x00fa, 0x0a);
        let prg: Vec<u8> = vec![0xe6, 0xfa, 00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.mem_read(0x00fa), 0x0a + 1);
    }

    #[test]
    fn test_iny() {
        let mut cpu = Cpu::new();
        cpu.reg.y = 0xef;
        let prg: Vec<u8> = vec![0xc8, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.y, 0xef + 1);
    }

    #[test]
    fn test_zeropage_x_ora() {
        let mut cpu = Cpu::new();
        cpu.reg.acc = 0xef;
        cpu.reg.x = 0x02;
        cpu.mem_write(0x00bd, 0x78);
        let prg: Vec<u8> = vec![0x15, 0xbd - 0x02, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0xef | 0x78);
    }

    #[test]
    fn test_pha() {
        let mut cpu = Cpu::new();
        cpu.reg.acc = 0xb4;
        let prg: Vec<u8> = vec![0x48, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.sp, 0xff - 1);
        assert_eq!(cpu.mem_read(0x01ff), 0xb4);
    }

    #[test]
    fn test_absolute_jmp() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0xf000 + 1, 0x00);
        let prg: Vec<u8> = vec![0x4c, 0x00, 0xf0, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.pc, 0xf000 + 1);
    }

    #[test]
    fn test_absolute_x_dec() {
        let mut cpu = Cpu::new();
        cpu.reg.x = 0x02;
        cpu.mem_write(0x01ff + cpu.reg.x as u16, 0x01);
        let prg: Vec<u8> = vec![0xde, 0xff, 0x01, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.mem_read(0x01ff + cpu.reg.x as u16), 0x00);
    }

    #[test]
    fn test_dey() {
        let mut cpu = Cpu::new();
        cpu.reg.y = 0xf1;
        let prg: Vec<u8> = vec![0x88, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.y, 0xf1 - 1);
    }

    #[test]
    fn test_pla() {
        let mut cpu = Cpu::new();
        cpu.mem_write(0x01fe, 0x01);
        cpu.reg.sp = 0xfe;
        let prg: Vec<u8> = vec![0x68, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.acc, 0x01);
        assert_eq!(cpu.reg.sp, 0xfe + 1);
    }
}

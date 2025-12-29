mod instructions;
mod registers;

use registers::{Registers, StatusFlags};

pub struct Cpu {
    reg: Registers,
    mem: Vec<u8>, // FIXME: probably must be fixed with the bus once done
    is_running: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let reg = Registers::init();
        Cpu {
            reg,
            mem: Vec::new(),
            is_running: false,
        }
    }

    pub fn load(&mut self, prg: Vec<u8>) {
        self.mem = prg;
    }

    pub fn run(&mut self) {
        self.reg.pc = 0; // FIXME: this line must be removed once the prg is loaded at tthe right
                         // location in memory

        self.is_running = true;
        while self.is_running {
            let op = self.mem[self.reg.pc as usize];
            self.reg.pc += 1;

            let instruction = Cpu::get_instruction_from_opcode(op);
            instruction.exec(self);
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
    // TODO: Need to check zero flag and negative flag for INX, how to ?
    fn test_inx() {
        let mut cpu = Cpu::new();
        let prg: Vec<u8> = vec![0xe8, 0x00];
        cpu.load(prg);
        cpu.run();
        assert_eq!(cpu.reg.x, 1);
    }
}

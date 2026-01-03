use crate::cpu::registers::StatusFlags;

use super::{AddressingMode, Cpu};

#[allow(non_snake_case, unused)]
impl Cpu {
    pub(super) fn BRK(&mut self, _: AddressingMode) {
        self.is_running = false;

        self.reg.set(StatusFlags::B);
        // TODO: there is stuff to do here with the flags, stack...
    }

    /// LDA loads a byte of memory into the accumulator.
    pub(super) fn LDA(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        self.reg.acc = operand;

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    /// TAX loads the accumulator into the X register.
    pub(super) fn TAX(&mut self, _: AddressingMode) {
        self.reg.x = self.reg.acc;

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }

    /// INX adds 1 to the X register.
    /// Note that it does not affect carry nor overflow flags.
    pub(super) fn INX(&mut self, _: AddressingMode) {
        self.reg.x = self.reg.x.wrapping_add(1);

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }

    /// An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
    pub(super) fn ORA(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        self.reg.acc |= operand;

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    /// This operation shifts all the bits of the accumulator or memory contents one bit left.
    /// Bit 0 is set to 0 and bit 7 is placed in the carry flag.
    /// The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.
    pub(super) fn ASL(&mut self, mode: AddressingMode) {
        let mut operand = self.get_instruction_operand(mode);
        let msb = operand & 0b1000_0000;

        operand <<= 1;

        if matches!(mode, AddressingMode::Accumulator) {
            self.reg.acc = operand;
        } else {
            // FIXME: This implies that the address is computed two times : one getting the operand
            // at the beginning of the method, and there... but whatever for now.
            let addr = self.get_operand_address(mode);

            self.mem_write(addr, operand);
        }

        self.set_carry(msb);
        self.set_zero(operand);
        self.set_negative(operand);
    }

    /// Pushes a copy of the status flags on to the stack.
    pub(super) fn PHP(&mut self, _: AddressingMode) {
        self.push_stack(self.reg.status | StatusFlags::B);
    }

    /// If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    pub(super) fn BPL(&mut self, mode: AddressingMode) {
        if self.reg.status & StatusFlags::NEGATIVE != 0 {
            return;
        }

        let relative_offset = self.get_instruction_operand(mode) as i8;

        self.reg.pc = self.reg.pc.wrapping_add(relative_offset as u16);
    }

    /// Set the carry flag to zero.
    pub(super) fn CLC(&mut self, _: AddressingMode) {
        self.reg.unset(StatusFlags::CARRY);
    }

    /// The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
    pub(super) fn JSR(&mut self, mode: AddressingMode) {
        const JSR_LENGTH: u16 = 3;

        let addr = self.reg.pc + 1;

        let lo = (addr & 0xff) as u8;
        let hi = (addr >> 8 & 0xff) as u8;

        self.push_stack(hi);
        self.push_stack(lo);

        let addr = self.get_operand_address(mode);

        // JSR_LENGTH must be substracted from the program counter due to the genericity (see,
        // CPU::execute_instruction) that **always** add the length of the instruction.
        self.reg.pc = addr - JSR_LENGTH;
    }

    /// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
    pub(super) fn AND(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        self.reg.acc &= operand;

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    /// This instructions is used to test if one or more bits are set in a target memory location.
    /// The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but the result is not kept.
    /// Bits 7 and 6 of the value from memory are copied into the N and V flags.
    pub(super) fn BIT(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        let result = operand & self.reg.acc;

        self.set_zero(result);

        if operand & 0b1000_0000 != 0 {
            self.reg.set(StatusFlags::NEGATIVE);
        } else {
            self.reg.unset(StatusFlags::NEGATIVE);
        }

        if operand & 0b0100_0000 != 0 {
            self.reg.set(StatusFlags::OVERFLOW);
        } else {
            self.reg.unset(StatusFlags::OVERFLOW);
        }
    }

    /// Move each of the bits in either A or M one place to the left.
    /// Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
    pub(super) fn ROL(&mut self, mode: AddressingMode) {
        let mut operand = self.get_instruction_operand(mode);
        let msb = operand & 0b1000_0000;

        operand <<= 1;

        if self.reg.status & StatusFlags::CARRY != 0 {
            operand |= 1;
        }

        if matches!(mode, AddressingMode::Accumulator) {
            self.reg.acc = operand;
        } else {
            let addr = self.get_operand_address(mode);

            self.mem_write(addr, operand);
        }

        self.set_carry(msb);
        self.set_zero(operand);
        self.set_negative(operand);
    }

    // Pulls an 8 bit value from the stack and into the processor flags.
    // The flags will take on new states as determined by the value pulled.
    pub(super) fn PLP(&mut self, _: AddressingMode) {
        // Note that PLP does ignore the B flag.
        self.reg.status = self.pull_stack() & !StatusFlags::B;

        // Ensure that the unused flag is always set to one.
        self.reg.status |= StatusFlags::UNUSED;
    }

    /// If the negative flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    pub(super) fn BMI(&mut self, mode: AddressingMode) {
        if self.reg.status & StatusFlags::NEGATIVE == 0 {
            return;
        }

        let relative_offset = self.get_instruction_operand(mode) as i8;

        self.reg.pc = self.reg.pc.wrapping_add(relative_offset as u16);
    }

    /// Set the carry flag to one.
    pub(super) fn SEC(&mut self, _: AddressingMode) {
        self.reg.set(StatusFlags::CARRY);
    }

    /// The RTI instruction is used at the end of an interrupt processing routine.
    /// It pulls the processor flags from the stack followed by the program counter.
    pub(super) fn RTI(&mut self, _: AddressingMode) {
        self.reg.status = self.pull_stack();

        let lo = self.pull_stack() as u16;
        let hi = self.pull_stack() as u16;

        self.reg.pc = (hi << 8) | lo;
    }

    /// An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
    pub(super) fn EOR(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        self.reg.acc ^= operand;

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    /// Each of the bits in A or M is shift one place to the right.
    /// The bit that was in bit 0 is shifted into the carry flag.
    /// Bit 7 is set to zero.
    pub(super) fn LSR(&mut self, mode: AddressingMode) {
        let mut operand = self.get_instruction_operand(mode);
        let lsb = operand & 1;

        operand >>= 1;

        if matches!(mode, AddressingMode::Accumulator) {
            self.reg.acc = operand;
        } else {
            let addr = self.get_operand_address(mode);

            self.mem_write(addr, operand);
        }

        self.set_carry(lsb);
        self.set_zero(operand);
        self.set_negative(operand);
    }

    /// Pushes a copy of the accumulator on to the stack.
    pub(super) fn PHA(&mut self, _: AddressingMode) {
        self.push_stack(self.reg.acc);
    }

    /// Sets the program counter to the address specified by the operand.
    pub(super) fn JMP(&mut self, mode: AddressingMode) {
        const JMP_LENGTH: u16 = 3;

        let addr = self.get_operand_address(mode);

        // JMP_LENGTH must be substracted from the program counter due to the genericity (see,
        // CPU::execute_instruction) that **always** add the length of the instruction.
        self.reg.pc = addr - JMP_LENGTH;
    }

    /// If the overflow flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    pub(super) fn BVC(&mut self, mode: AddressingMode) {
        if self.reg.status & StatusFlags::OVERFLOW != 0 {
            return;
        }

        let relative_offset = self.get_instruction_operand(mode) as i8;

        self.reg.pc = self.reg.pc.wrapping_add(relative_offset as u16);
    }

    /// Clears the interrupt disable flag allowing normal interrupt requests to be serviced.
    pub(super) fn CLI(&mut self, _: AddressingMode) {
        self.reg.unset(StatusFlags::INTERRUPT_DISABLE);
    }

    /// The RTS instruction is used at the end of a subroutine to return to the calling routine.
    /// It pulls the program counter (minus one) from the stack.
    pub(super) fn RTS(&mut self, _: AddressingMode) {
        let lo = self.pull_stack() as u16;
        let hi = self.pull_stack() as u16;

        let addr = (hi << 8) | lo;

        self.reg.pc = addr + 1;
    }

    /// This instruction adds the contents of a memory location to the accumulator together with the carry bit.
    /// If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
    pub(super) fn ADC(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);
        let mut flag_carry = false;

        let (mut result, operand_carry) = self.reg.acc.overflowing_add(operand);

        if self.reg.status & StatusFlags::CARRY != 0 {
            (result, flag_carry) = result.overflowing_add(1);
            self.reg.unset(StatusFlags::CARRY);
        }

        if operand_carry || flag_carry {
            self.reg.set(StatusFlags::CARRY);
        } else {
            self.reg.unset(StatusFlags::CARRY);
        }

        if (result ^ self.reg.acc) & (result ^ operand) & 0b1000_0000 != 0 {
            self.reg.set(StatusFlags::OVERFLOW);
        } else {
            self.reg.unset(StatusFlags::OVERFLOW);
        }

        self.reg.acc = result;
        self.set_zero(result);
        self.set_negative(result);
    }

    /// Move each of the bits in either A or M one place to the right.
    /// Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
    pub(super) fn ROR(&mut self, mode: AddressingMode) {
        let mut operand = self.get_instruction_operand(mode);
        let lsb = operand & 1;

        operand >>= 1;

        if self.reg.status & StatusFlags::CARRY != 0 {
            operand |= 0b1000_0000;
        }

        if matches!(mode, AddressingMode::Accumulator) {
            self.reg.acc = operand;
        } else {
            let addr = self.get_operand_address(mode);

            self.mem_write(addr, operand);
        }

        self.set_carry(lsb);
        self.set_zero(operand);
        self.set_negative(operand);
    }

    /// Pulls an 8 bit value from the stack and into the accumulator.
    /// The zero and negative flags are set as appropriate.
    pub(super) fn PLA(&mut self, _: AddressingMode) {
        self.reg.acc = self.pull_stack();

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    pub(super) fn BVS(&mut self, mode: AddressingMode) {
        if self.reg.status & StatusFlags::OVERFLOW == 0 {
            return;
        }

        let relative_offset = self.get_instruction_operand(mode) as i8;

        self.reg.pc = self.reg.pc.wrapping_add(relative_offset as u16);
    }

    /// Set the interrupt disable flag to one.
    pub(super) fn SEI(&mut self, _: AddressingMode) {
        self.reg.set(StatusFlags::INTERRUPT_DISABLE);
    }

    /// Stores the contents of the accumulator register into memory.
    pub(super) fn STA(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.reg.acc);
    }

    /// Stores the contents of the Y register into memory.
    pub(super) fn STY(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.reg.y);
    }

    /// Stores the contents of the X register into memory.
    pub(super) fn STX(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);

        self.mem_write(addr, self.reg.x);
    }

    /// Subtracts one from the Y register setting the zero and negative flags as appropriate.
    pub(super) fn DEY(&mut self, _: AddressingMode) {
        self.reg.y -= 1;

        self.set_zero(self.reg.y);
        self.set_negative(self.reg.y);
    }

    /// Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.
    pub(super) fn TXA(&mut self, _: AddressingMode) {
        self.reg.acc = self.reg.x;

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    /// If the carry flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    pub(super) fn BCC(&mut self, mode: AddressingMode) {
        if self.reg.status & StatusFlags::CARRY != 0 {
            return;
        }

        let relative_offset = self.get_instruction_operand(mode) as i8;

        self.reg.pc = self.reg.pc.wrapping_add(relative_offset as u16);
    }

    /// Copies the current contents of the Y register into the accumulator and sets the zero and negative flags as appropriate.
    pub(super) fn TYA(&mut self, _: AddressingMode) {
        self.reg.acc = self.reg.y;

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    /// Copies the current contents of the X register into the stack register.
    pub(super) fn TXS(&mut self, _: AddressingMode) {
        self.reg.sp = self.reg.x;
    }

    /// Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.
    pub(super) fn LDY(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        self.reg.y = operand;

        self.set_zero(self.reg.y);
        self.set_negative(self.reg.y);
    }

    /// Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
    pub(super) fn LDX(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        self.reg.x = operand;

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }

    /// Copies the current contents of the accumulator into the Y register and sets the zero and negative flags as appropriate.
    pub(super) fn TAY(&mut self, _: AddressingMode) {
        self.reg.y = self.reg.acc;

        self.set_zero(self.reg.y);
        self.set_negative(self.reg.y);
    }

    /// If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    pub(super) fn BCS(&mut self, mode: AddressingMode) {
        if self.reg.status & StatusFlags::CARRY == 0 {
            return;
        }

        let relative_offset = self.get_instruction_operand(mode) as i8;

        self.reg.pc = self.reg.pc.wrapping_add(relative_offset as u16);
    }

    /// Clears the overflow flag.
    pub(super) fn CLV(&mut self, _: AddressingMode) {
        self.reg.unset(StatusFlags::OVERFLOW);
    }

    /// Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.
    pub(super) fn TSX(&mut self, _: AddressingMode) {
        self.reg.x = self.reg.sp;

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }

    /// This instruction compares the contents of the Y register with another memory held value and sets the zero and carry flags as appropriate.
    pub(super) fn CPY(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        let result = self.reg.y.wrapping_sub(operand);

        if self.reg.y >= operand {
            self.reg.set(StatusFlags::CARRY);
        } else {
            self.reg.unset(StatusFlags::CARRY);
        }

        self.set_zero(result);
        self.set_negative(result);
    }

    /// This instruction compares the contents of the accumulator with another memory held value and sets the zero and carry flags as appropriate.
    pub(super) fn CMP(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        let result = self.reg.acc.wrapping_sub(operand);

        if self.reg.acc >= operand {
            self.reg.set(StatusFlags::CARRY);
        } else {
            self.reg.unset(StatusFlags::CARRY);
        }

        self.set_zero(result);
        self.set_negative(result);
    }

    /// Subtracts one from the value held at a specified memory location setting the zero and negative flags as appropriate.
    pub(super) fn DEC(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);

        let data = self.mem_read(addr) - 1;

        self.mem_write(addr, data);

        self.set_zero(data);
        self.set_negative(data);
    }

    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    pub(super) fn INY(&mut self, _: AddressingMode) {
        self.reg.y = self.reg.y.wrapping_add(1);

        self.set_zero(self.reg.y);
        self.set_negative(self.reg.y);
    }

    /// Subtracts one from the X register setting the zero and negative flags as appropriate.
    pub(super) fn DEX(&mut self, _: AddressingMode) {
        self.reg.x -= 1;

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }

    /// If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    pub(super) fn BNE(&mut self, mode: AddressingMode) {
        if self.reg.status & StatusFlags::ZERO != 0 {
            return;
        }

        let relative_offset = self.get_instruction_operand(mode) as i8;

        self.reg.pc = self.reg.pc.wrapping_add(relative_offset as u16);
    }

    /// Sets the decimal mode flag to zero.
    pub(super) fn CLD(&mut self, _: AddressingMode) {
        self.reg.unset(StatusFlags::DECIMAL);
    }

    /// This instruction compares the contents of the X register with another memory held value
    /// and sets the zero and carry flags as appropriate.
    pub(super) fn CPX(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);

        let result = self.reg.x.wrapping_sub(operand);

        if self.reg.x >= operand {
            self.reg.set(StatusFlags::CARRY);
        } else {
            self.reg.unset(StatusFlags::CARRY);
        }

        self.set_zero(result);
        self.set_negative(result);
    }

    /// This instruction subtracts the contents of a memory location to the accumulator together with the not of the carry bit.
    /// If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.
    pub(super) fn SBC(&mut self, mode: AddressingMode) {
        let operand = self.get_instruction_operand(mode);
        let mut flag_carry = false;

        let (mut result, operand_carry) = self.reg.acc.overflowing_sub(operand);

        if self.reg.status & StatusFlags::CARRY == 0 {
            (result, flag_carry) = result.overflowing_sub(1);
            self.reg.unset(StatusFlags::CARRY);
        }

        if operand_carry || flag_carry {
            self.reg.unset(StatusFlags::CARRY);
        } else {
            self.reg.set(StatusFlags::CARRY);
        }

        if (result ^ self.reg.acc) & (result ^ !operand) & 0b1000_0000 != 0 {
            self.reg.set(StatusFlags::OVERFLOW);
        } else {
            self.reg.unset(StatusFlags::OVERFLOW);
        }

        self.reg.acc = result;
        self.set_zero(result);
        self.set_negative(result);
    }

    /// Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.
    pub(super) fn INC(&mut self, mode: AddressingMode) {
        let addr = self.get_operand_address(mode);

        let data = self.mem_read(addr) + 1;

        self.mem_write(addr, data);

        self.set_zero(data);
        self.set_negative(data);
    }

    /// The NOP instruction causes no changes to the processor other than the normal incrementing
    /// of the program counter to the next instruction.
    pub(super) fn NOP(&mut self, _: AddressingMode) {}

    /// If the zero flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    pub(super) fn BEQ(&mut self, mode: AddressingMode) {
        if self.reg.status & StatusFlags::ZERO == 0 {
            return;
        }

        let relative_offset = self.get_instruction_operand(mode) as i8;

        self.reg.pc = self.reg.pc.wrapping_add(relative_offset as u16);
    }

    /// Set the decimal mode flag to one.
    pub(super) fn SED(&mut self, _: AddressingMode) {
        self.reg.set(StatusFlags::DECIMAL);
    }
}

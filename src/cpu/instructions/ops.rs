use super::{AddressingMode, Cpu};

#[allow(non_snake_case)]
impl Cpu {
    pub(super) fn BRK(&mut self, _mode: AddressingMode) {
        self.is_running = false;
        // TODO: there is stuff to do here with the flags...
    }

    /// LDA loads a byte of memory into the accumulator.
    pub(super) fn LDA(&mut self, mode: AddressingMode) {
        let operand = self.read_operand(mode);

        self.reg.acc = operand;

        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

    /// TAX loads the accumulator into the X register.
    pub(super) fn TAX(&mut self, _mode: AddressingMode) {
        self.reg.x = self.reg.acc;

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }

    /// INX adds 1 to the X register.
    /// Note that it does not affect carry nor overflow flags.
    pub(super) fn INX(&mut self, _mode: AddressingMode) {
        self.reg.x = self.reg.x.wrapping_add(1);

        self.set_zero(self.reg.x);
        self.set_negative(self.reg.x);
    }

    pub(super) fn ORA(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn ASL(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn PHP(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BPL(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn CLC(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn JSR(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn AND(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BIT(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn ROL(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn PLP(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BMI(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn SEC(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn RTI(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn EOR(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn LSR(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn PHA(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn JMP(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BVC(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn CLI(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn RTS(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn ADC(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn ROR(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn PLA(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BVS(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn SEI(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn STA(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn STY(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn STX(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn DEY(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn TXA(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BCC(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn TYA(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn TXS(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn LDY(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn LDX(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn TAY(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BCS(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn CLV(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn TSX(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn CPY(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn CMP(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn DEC(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn INY(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn DEX(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BNE(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn CLD(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn CPX(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn SBC(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn INC(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn NOP(&mut self, _mode: AddressingMode) {
        todo!()
    }

    pub(super) fn BEQ(&mut self, mode: AddressingMode) {
        todo!()
    }

    pub(super) fn SED(&mut self, _mode: AddressingMode) {
        todo!()
    }
}

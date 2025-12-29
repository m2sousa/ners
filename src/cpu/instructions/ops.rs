use super::{AddressingMode, Cpu};

#[allow(non_snake_case)]
impl Cpu {
    pub(super) fn BRK(&mut self, _mode: AddressingMode) {
        self.is_running = false;
        // TODO: there is stuff to do here with the flags...
    }

    pub(super) fn LDA(&mut self, mode: AddressingMode) {
        let value = self.read_operand(mode);

        self.reg.acc = value;
        self.set_zero(self.reg.acc);
        self.set_negative(self.reg.acc);
    }

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
}

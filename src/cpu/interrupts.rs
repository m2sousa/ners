use super::{Bus, Cpu, StatusFlags};

impl Cpu {
    pub fn non_maskable_interrupt(&mut self, bus: &mut Bus) {
        const NMI_VECTOR: u16 = 0xfffa;

        let hi: u8 = (self.reg.pc >> 8) as u8;
        let lo: u8 = self.reg.pc as u8;

        self.push_stack(bus, hi);
        self.push_stack(bus, lo);

        // Note that the B flag will be set to 0 there.
        self.push_stack(bus, self.reg.status & !StatusFlags::B);

        let lo = self.mem_read(bus, NMI_VECTOR) as u16;
        let hi = self.mem_read(bus, NMI_VECTOR + 1) as u16;

        self.reg.pc = (hi << 8) | lo;
    }

    pub fn interrupt_request(&mut self, bus: &mut Bus) {
        const IRQ_VECTOR: u16 = 0xfffe;

        todo!();
    }

    // TODO: Some work must be done there in the reset methods, notably assigning the stack pointer
    // and some flags...
    pub fn reset(&mut self, bus: &mut Bus) {
        const RESET_VECTOR: u16 = 0xfffc;

        let lo = self.mem_read(bus, RESET_VECTOR) as u16;
        let hi = self.mem_read(bus, RESET_VECTOR + 1) as u16;

        self.reg.pc = (hi << 8) | lo;
        println!("[DBG] Program counter set to 0x{:4X}.", self.reg.pc);
    }
}

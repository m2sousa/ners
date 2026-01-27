pub struct Bus {
    ram: [u8; u16::MAX as usize + 1],
}

impl Bus {
    const CPU_RAM_SIZE: usize = 2048;

    pub fn new() -> Self {
        Bus {
            ram: [0; u16::MAX as usize + 1],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}

use crate::loader::{Loader, RomLoader};

pub struct Bus {
    ram: [u8; u16::MAX as usize + 1],
    rom: Vec<u8>,
}

impl Bus {
    const ROM_START: usize = 0x8000;
    const ROM_END: usize = 0xffff;

    pub fn new() -> Self {
        Bus {
            ram: [0; u16::MAX as usize + 1],
            // Initialize with 16KB of memory for the PRG-ROM.
            rom: Vec::with_capacity(16384),
        }
    }

    pub fn load_rom_data(&mut self, loader: RomLoader) {
        self.rom = loader.get_prg_rom();
        println!("[DBG] Loaded {} bytes of data in the rom.", self.rom.len());
    }

    pub fn read(&self, addr: u16) -> u8 {
        let mut addr = addr as usize;
        match addr {
            Self::ROM_START..=Self::ROM_END => {
                addr -= Self::ROM_START;

                // Some roms are 16KB long, and thus, in this case, the 32KB of the ROM space are mirrored.
                if self.rom.len() == 16384 {
                    addr %= 16384;
                }

                self.rom[addr]
            }
            _ => self.ram[addr],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
}

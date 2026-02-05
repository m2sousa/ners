use crate::loader::RomLoader;

pub struct Bus {
    wram: [u8; 2048],
    ppu_registers: [u8; 8],
    rom: Vec<u8>,
}

impl Bus {
    const CPU_RAM_START: u16 = 0x0000;
    const CPU_RAM_END: u16 = 0x1fff;

    const ROM_START: u16 = 0x8000;
    const ROM_END: u16 = 0xffff;

    const PPU_REGISTERS_START: u16 = 0x2000;
    const PPU_REGISTERS_END: u16 = 0x3fff;

    pub fn new() -> Self {
        Bus {
            wram: [0; 2048],
            ppu_registers: [0; 8],
            // Initialize with 16KB of memory for the PRG-ROM.
            rom: Vec::with_capacity(16384),
        }
    }

    pub fn load_prg_data(&mut self, loader: &RomLoader) {
        self.rom = loader.get_prg_rom();
        println!("[DBG] Loaded {} bytes of data in the rom.", self.rom.len());
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            Self::CPU_RAM_START..=Self::CPU_RAM_END => self.wram[self.apply_mirroring(addr)],
            Self::PPU_REGISTERS_START..=Self::PPU_REGISTERS_END => {
                self.ppu_registers[self.apply_mirroring(addr)]
            }
            Self::ROM_START..=Self::ROM_END => self.rom[self.apply_mirroring(addr)],
            _ => todo!("cannot read memory space at 0x{:4X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            Self::CPU_RAM_START..=Self::CPU_RAM_END => self.wram[self.apply_mirroring(addr)] = data,
            Self::PPU_REGISTERS_START..=Self::PPU_REGISTERS_END => {
                self.ppu_registers[self.apply_mirroring(addr)] = data
            }
            Self::ROM_START..=Self::ROM_END => {
                panic!("[ERR] Trying to write on the ROM at address 0x{:04X}", addr)
            }
            _ => todo!("cannot write memory space at 0x{:4X}", addr),
        }
    }

    fn apply_mirroring(&self, addr: u16) -> usize {
        let mut addr = addr;
        match addr {
            Self::CPU_RAM_START..=Self::CPU_RAM_END => {
                addr %= 2048;
            }
            Self::ROM_START..=Self::ROM_END => {
                addr -= Self::ROM_START;

                // Some roms are 16KB long, and thus, in this case, the 32KB of the ROM space are mirrored.
                if self.rom.len() == 16384 {
                    addr %= 16384;
                }
            }
            Self::PPU_REGISTERS_START..=Self::PPU_REGISTERS_END => {
                // PPU I/O registers are mirronred every 8 bytes on the given memory space.
                addr %= 8
            }
            _ => {}
        }

        addr as usize
    }
}

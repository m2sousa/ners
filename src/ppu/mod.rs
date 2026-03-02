mod mmio;
mod oam;
mod pattern;

use crate::loader::RomLoader;

use mmio::PpuStatus;
use pattern::PatternTables;

#[derive(Copy, Clone)]
pub enum NametableArrangement {
    Vertical,
    Horizontal,
}

pub struct Ppu {
    reg: mmio::Registers,

    // Note that the PPU and CPU are on different buses,
    // thus we define PPU memory there. A bus struct could
    // be used as well, but seems overkill for now.
    vram: [u8; Ppu::PPU_RAM_SIZE],
    palettes: [u8; Ppu::PALETTES_SIZE],
    object_attribute_mem: [u8; Ppu::OAM_MEM_SIZE],
    pattern_table: Option<PatternTables>,

    nametable_arrangement: Option<NametableArrangement>,

    data_buffer: u8,
    addr: u16,
    addr_latch: bool, // w register

    current_scanline: usize,
    current_dot: usize,

    nmi_pending: bool,
}

impl Ppu {
    /// Ppu has access to 2 Kilobytes of RAM on its own address space.
    const PPU_RAM_SIZE: usize = 2048;
    const RAM_ADDR_START: u16 = 0x2000;
    const RAM_ADDR_END: u16 = 0x2fff;

    /// The Ppu internally contains those 256 bytes of memory to determine how sprite are rendered.
    /// There is 64 entries of 4 bytes long, thus 256 bytes.
    const OAM_MEM_SIZE: usize = 256;

    /// The Ppu has 4 palettes for the background, and 4 palettes for the foreground.
    /// Thus, totaling to 8 palettes of 4 bytes (colors), 32 bytes of memory in total.
    const PALETTES_SIZE: usize = 32;
    const PALETTE_ADDR_START: u16 = 0x3f00;
    const PALETTE_ADDR_END: u16 = 0x3fff;

    const PATTERNS_ADDR_START: u16 = 0x0000;
    const PATTERNS_ADDR_END: u16 = 0x1fff;

    pub fn new() -> Self {
        let reg = mmio::Registers::new();

        let vram = [0; Ppu::PPU_RAM_SIZE];
        let palettes = [0; Ppu::PALETTES_SIZE];
        let object_attribute_mem = [0; Ppu::OAM_MEM_SIZE];

        Ppu {
            reg,
            vram,
            palettes,
            object_attribute_mem,
            pattern_table: None,

            nametable_arrangement: None,

            data_buffer: 0x00,
            addr: 0x0000,
            addr_latch: false,

            current_scanline: 0,
            current_dot: 0,

            nmi_pending: false,
        }
    }

    // FIXME: I should probably change the name of this api function...
    pub fn load_chr_data(&mut self, loader: &RomLoader) {
        let data = &loader.get_chr_rom();
        self.pattern_table = Some(PatternTables::new(data));
        println!(
            "[DBG] Loaded {} bytes of data in the pattern table.",
            data.len()
        );

        let nametable_arrangement = loader.get_nametable_arrangement();
        self.nametable_arrangement = Some(nametable_arrangement);
    }

    pub fn read_register(&mut self, reg: usize) -> u8 {
        match reg {
            mmio::PPUSTATUS => {
                // On reads of PPUSTATUS, the latch must be cleared.
                self.addr_latch = false;
            }
            mmio::PPUDATA => {
                return self.mem_read();
            }
            _ => {}
        }

        self.reg.read(reg)
    }

    pub fn write_register(&mut self, reg: usize, data: u8) {
        match reg {
            mmio::PPUADDR => {
                let data = data as u16;

                if !self.addr_latch {
                    self.addr = (self.addr & 0x00FF) | (data << 8);
                } else {
                    self.addr = (self.addr & 0xFF00) | data;
                }

                self.addr &= 0x3FFF;
                self.addr_latch = !self.addr_latch;
            }
            mmio::PPUDATA => {
                self.mem_write(data);
                return;
            }
            _ => {}
        }

        self.reg.write(reg, data);
    }

    pub fn step(&mut self, cycles: usize) {
        const MAX_DOTS: usize = 341;
        const MAX_SCANLINE: usize = 262;
        const VBLANK_SCANLINE: usize = 241;

        self.current_dot += cycles;
        if self.current_dot >= MAX_DOTS {
            self.current_dot -= MAX_DOTS;
            self.current_scanline += 1;

            if self.current_scanline == VBLANK_SCANLINE {
                self.reg.set_status(PpuStatus::VBLANK_FLAG);
                if self.reg.is_nmi_enabled() {
                    self.nmi_pending = true;
                }
            }

            if self.current_scanline >= MAX_SCANLINE {
                self.reg.unset_status(PpuStatus::VBLANK_FLAG);
                self.current_scanline = 0;
            }
        }
    }

    pub fn poll_nmi(&mut self) -> bool {
        std::mem::replace(&mut self.nmi_pending, false)
    }

    fn mem_read(&mut self) -> u8 {
        let data = match self.addr {
            Self::PATTERNS_ADDR_START..=Self::PATTERNS_ADDR_END => {
                let delayed_data = self.data_buffer;

                // FIXME: Is this really ok ? It provides the uncompressed data (i.e. index in the
                // palette for the given dot) due to the Index trait, rather than the raw byte data ?
                let idx = (self.addr as usize & 0x1000) >> 12;
                self.data_buffer = self.pattern_table.as_ref().unwrap()[idx][self.addr as usize];

                delayed_data
            }
            Self::RAM_ADDR_START..=Self::RAM_ADDR_END => {
                let delayed_data = self.data_buffer;

                let addr = (self.addr - Self::RAM_ADDR_START) as usize;
                self.data_buffer = self.vram[addr];

                delayed_data
            }
            Self::PALETTE_ADDR_START..=Self::PALETTE_ADDR_END => {
                // There is no dummy reads when reading the palette data.
                let mut addr = (self.addr - Self::PALETTE_ADDR_START) as usize;
                addr %= Self::PALETTES_SIZE;
                self.data_buffer = self.palettes[addr];

                self.data_buffer
            }
            _ => unimplemented!(
                "[ERR] Invalid PPU memory access to read : 0x{:4x}",
                self.addr
            ),
        };

        self.addr += self.reg.get_addr_increment();
        data
    }

    fn mem_write(&mut self, data: u8) {
        match self.addr {
            Self::PATTERNS_ADDR_START..=Self::PATTERNS_ADDR_END => {
                // FIXME: Should it write in the pattern table in the case of CHR-RAM ?
                panic!("FIXME: write to pattern tables ?");
            }
            Self::RAM_ADDR_START..=Self::RAM_ADDR_END => {
                let mut addr = (self.addr - Self::RAM_ADDR_START) as usize;
                // TODO: mirroring with cartridge information must be done here...
                addr %= Self::PPU_RAM_SIZE;
                self.vram[addr] = data;
            }
            Self::PALETTE_ADDR_START..=Self::PALETTE_ADDR_END => {
                let addr = (self.addr - Self::PALETTE_ADDR_START) as usize;
                self.palettes[addr] = data;
            }
            _ => unimplemented!(
                "[ERR] Invalid PPU memory access to write : 0x{:4x}",
                self.addr
            ),
        }

        self.addr += self.reg.get_addr_increment();
    }

    pub(super) fn get_decoded_pattern_table(&self) -> (&[u8], &[u8]) {
        self.pattern_table
            .as_ref()
            .unwrap() // This unwrap should not panic in any case.
            .get_decoded_pattern_table()
    }

    pub(super) fn get_palettes(&self) -> &[u8] {
        &self.palettes
    }
}

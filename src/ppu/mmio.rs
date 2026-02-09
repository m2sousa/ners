#[rustfmt::skip]
#[allow(non_snake_case)]
pub(super) mod PpuCtrl {
    pub const NAMETABLE_BASE: u8        = 0b0000_0011;
    pub const VRAM_INCREMENT: u8        = 0b0000_0100;
    pub const SPRITE_PATTERN: u8        = 0b0000_1000;

    pub const BACKGROUND_PATTERN: u8    = 0b0001_0000;
    pub const SPRITE_SIZE: u8           = 0b0010_0000;
    pub const PPU_MASTER_SELECT: u8     = 0b0100_0000;
    pub const VBLANK_NMI_ENABLE: u8     = 0b1000_0000;
}

#[rustfmt::skip]
#[allow(non_snake_case)]
pub(super) mod PpuMask {
    pub const GREYSCALE: u8             = 0b0000_0001;
    pub const BACKGROUND_LEFTMOST: u8   = 0b0000_0010;
    pub const SPRITE_LEFTMOST: u8       = 0b0000_0100;
    pub const BACKGROUND_RENDERING: u8  = 0b0000_1000;

    pub const SPRITE_RENDERING: u8      = 0b0001_0000;
    pub const EMPHASIZE_RED: u8         = 0b0010_0000;
    pub const EMPHASIZE_GREEN: u8       = 0b0100_0000;
    pub const EMPHASIZE_BLUE: u8        = 0b1000_0000;
}

#[rustfmt::skip]
#[allow(non_snake_case)]
pub(super) mod PpuStatus {
    pub const PPU_OPEN_BUS: u8          = 0b0001_1111;
    pub const SPRITE_OVERFLOW: u8       = 0b0010_0000;
    pub const SPRITE_ZERO_HIT: u8       = 0b0100_0000;
    pub const VBLANK_FLAG: u8           = 0b1000_0000;
}

pub(super) const PPUCTRL: usize = 0x2000;
pub(super) const PPUMASK: usize = 0x2001;
pub(super) const PPUSTATUS: usize = 0x2002;
pub(super) const OAMADDR: usize = 0x2003;
pub(super) const OAMDATA: usize = 0x2004;
pub(super) const PPUSCROLL: usize = 0x2005;
pub(super) const PPUADDR: usize = 0x2006;
pub(super) const PPUDATA: usize = 0x2007;
pub(super) const OAMDMA: usize = 0x4014;

#[derive(Default)]
pub(super) struct Registers {
    ctrl: u8,
    mask: u8,
    status: u8,
    oamaddr: u8,
    oamdata: u8,
    scroll: u8,
    addr: u8,
    data: u8,
    dma: u8,
}

impl Registers {
    pub(super) fn new() -> Self {
        Registers {
            status: 0b1010_0000,
            ..Default::default()
        }
    }

    pub(super) fn read(&mut self, reg: usize) -> u8 {
        match reg {
            PPUCTRL => self.ctrl,
            PPUMASK => self.mask,
            PPUSTATUS => {
                let current_status = self.status;
                // FIXME: Unset the VBLANK_FLAG on reads...
                // self.status &= !PpuStatus::VBLANK_FLAG;
                current_status
            }
            OAMADDR => self.oamaddr,
            OAMDATA => self.oamdata,
            PPUSCROLL => self.scroll,
            PPUADDR => self.addr,
            PPUDATA => self.data,
            OAMDMA => self.dma,
            _ => unreachable!("[ERR] Unknown PPU register read access : 0x{:4x}", reg),
        }
    }

    pub(super) fn write(&mut self, reg: usize, data: u8) {
        match reg {
            PPUCTRL => self.ctrl = data,
            PPUMASK => self.mask = data,
            PPUSTATUS => self.status = data,
            OAMADDR => self.oamaddr = data,
            OAMDATA => self.oamdata = data,
            PPUSCROLL => self.scroll = data,
            PPUADDR => self.addr = data,
            PPUDATA => self.data = data,
            OAMDMA => self.dma = data,
            _ => unreachable!("[ERR] Unknown PPU register write access : 0x{:4x}", reg),
        }
    }

    pub(super) fn get_addr_increment(&self) -> u16 {
        if (self.ctrl & PpuCtrl::VRAM_INCREMENT) != 0 {
            32
        } else {
            1
        }
    }
}

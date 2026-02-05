#[rustfmt::skip]
#[allow(non_snake_case)]
pub mod PpuCtrl {
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
pub mod PpuMask {
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
pub mod PpuStatus {
    pub const PPU_OPEN_BUS: u8      = 0b0001_1111;
    pub const SPRITE_OVERFLOW: u8   = 0b0010_0000;
    pub const SPRITE_ZERO_HIT: u8   = 0b0100_0000;
    pub const VBLANK_FLAG: u8       = 0b1000_0000;
}

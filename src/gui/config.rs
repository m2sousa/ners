pub(super) struct WindowConfig {
    pub tile_size: usize,
    pub tiles_per_row: usize,

    pub scale: u32,
    pub padding: usize,
}

impl WindowConfig {
    pub(super) fn new() -> Self {
        WindowConfig {
            tile_size: 8,
            scale: 2,
            tiles_per_row: 16,
            padding: 10,
        }
    }

    pub(super) fn width(&self) -> u32 {
        (self.tiles_per_row * self.tile_size * self.scale as usize * 2 + self.padding) as u32
    }

    pub(super) fn height(&self) -> u32 {
        (self.tiles_per_row * self.tile_size * self.scale as usize) as u32
            + self.palette_section_height()
    }

    pub(super) fn right_table_offset_x(&self) -> i32 {
        (self.tiles_per_row * self.tile_size * self.scale as usize + self.padding) as i32
    }

    pub(super) fn pattern_table_y_offset(&self) -> i32 {
        self.palette_section_height() as i32
    }

    pub(super) fn palette_gap(&self) -> u32 {
        4
    }

    pub(super) fn palette_swatch_size(&self) -> u32 {
        (self.width() - self.palette_gap() * 7) / 32
    }

    pub(super) fn palette_section_height(&self) -> u32 {
        self.palette_swatch_size() + self.padding as u32 * 2
    }
}

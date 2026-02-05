mod mmio;

use crate::loader::RomLoader;

pub struct Ppu {
    // Note that the PPU and CPU are on different buses,
    // thus we define the video ram for the ppu there.
    // Should I use a PpuBus struct ?
    vram: [u8; 2048],
    pattern_table: Option<Vec<u8>>,
    palettes: [u8; 8 * 4],
    // object_attribute_mem
    // palette_mem
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: [0; 2048],
            pattern_table: None,
            palettes: [0; 32],
        }
    }

    pub fn load_chr_data(&mut self, loader: &RomLoader) {
        self.pattern_table = Some(loader.get_chr_rom());
        println!(
            "[DBG] Loaded {} bytes of data in the pattern table.",
            self.pattern_table.as_ref().unwrap().len()
        );
    }
}

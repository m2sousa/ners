extern crate sdl2;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use super::NersGui;
use crate::gui::colors;
use crate::ners::Ners;

impl NersGui {
    pub(super) fn render(&self, canvas: &mut Canvas<Window>, core: &Ners) {
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();

        let (left_table, right_table) = core.get_decoded_pattern_table();
        let palette_data = core.get_palettes();
        let active_palette =
            &palette_data[self.current_palette_idx * 4..(self.current_palette_idx + 1) * 4];

        let total_palette_width =
            8 * 4 * self.config.palette_swatch_size() + 7 * self.config.palette_gap();
        let palette_offset_x = (self.config.width() - total_palette_width) / 2;

        self.draw_palettes(
            canvas,
            &palette_data,
            palette_offset_x as i32,
            self.config.padding as i32,
        );

        let y_offset = self.config.pattern_table_y_offset();
        self.draw_pattern_table(canvas, left_table, 0, y_offset, active_palette);
        self.draw_pattern_table(
            canvas,
            right_table,
            self.config.right_table_offset_x(),
            y_offset,
            active_palette,
        );

        canvas.present();
    }

    fn draw_pattern_table(
        &self,
        canvas: &mut Canvas<Window>,
        pattern_table: &[u8],
        offset_x: i32,
        offset_y: i32,
        palette: &[u8],
    ) {
        const TILES_PER_ROW: usize = 16;
        const TILE_SIZE: usize = 8;
        let scale = self.config.scale as i32;

        for tile_idx in 0..256usize {
            let tile_x = (tile_idx % TILES_PER_ROW) as i32;
            let tile_y = (tile_idx / TILES_PER_ROW) as i32;
            for pixel_idx in 0..64usize {
                let pixel_x = (pixel_idx % TILE_SIZE) as i32;
                let pixel_y = (pixel_idx / TILE_SIZE) as i32;
                let color_idx = pattern_table[tile_idx * 64 + pixel_idx] as usize;
                canvas.set_draw_color(colors::get_rgb_from_idx(palette[color_idx]));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        offset_x + (tile_x * TILE_SIZE as i32 + pixel_x) * scale,
                        offset_y + (tile_y * TILE_SIZE as i32 + pixel_y) * scale,
                        self.config.scale,
                        self.config.scale,
                    ))
                    .unwrap();
            }
        }
    }

    fn draw_palettes(
        &self,
        canvas: &mut Canvas<Window>,
        palette_data: &[u8],
        offset_x: i32,
        offset_y: i32,
    ) {
        const NUM_PALETTES: usize = 8;
        const COLORS_PER_PALETTE: usize = 4;
        const BORDER: i32 = 2;

        let swatch_size = self.config.palette_swatch_size();
        let group_gap = self.config.palette_gap() as i32;

        for palette_idx in 0..NUM_PALETTES {
            let group_x = offset_x
                + palette_idx as i32 * (COLORS_PER_PALETTE as i32 * swatch_size as i32 + group_gap);

            if palette_idx == self.current_palette_idx {
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        group_x - BORDER,
                        offset_y - BORDER,
                        COLORS_PER_PALETTE as u32 * swatch_size + BORDER as u32 * 2,
                        swatch_size + BORDER as u32 * 2,
                    ))
                    .unwrap();
            }

            for color_idx in 0..COLORS_PER_PALETTE {
                let nes_color = palette_data[palette_idx * COLORS_PER_PALETTE + color_idx];
                canvas.set_draw_color(colors::get_rgb_from_idx(nes_color));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        group_x + color_idx as i32 * swatch_size as i32,
                        offset_y,
                        swatch_size,
                        swatch_size,
                    ))
                    .unwrap();
            }
        }
    }
}

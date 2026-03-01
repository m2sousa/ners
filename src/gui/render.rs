extern crate sdl2;
use sdl2::{pixels::Color, render::Canvas, video::Window};

use crate::gui::colors;

use super::NersGui;

impl NersGui {
    pub(super) fn draw_pattern_table(
        canvas: &mut Canvas<Window>,
        pattern_table: &[u8],
        offset_x: i32,
        offset_y: i32,
        scale: u32,
        palette_data: &[u8],
    ) {
        const TILES_PER_ROW: usize = 16;
        const TILE_SIZE: usize = 8;

        for tile_idx in 0..256 {
            let tile_x = tile_idx % TILES_PER_ROW;
            let tile_y = tile_idx / TILES_PER_ROW;

            for pixel_idx in 0..64 {
                let pixel_x = pixel_idx % TILE_SIZE;
                let pixel_y = pixel_idx / TILE_SIZE;

                let color_idx = pattern_table[tile_idx * 64 + pixel_idx] as usize;
                let color = colors::get_rgb_from_idx(palette_data[color_idx]);

                canvas.set_draw_color(color);

                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        offset_x + (tile_x * TILE_SIZE + pixel_x) as i32 * scale as i32,
                        offset_y + (tile_y * TILE_SIZE + pixel_y) as i32 * scale as i32,
                        scale,
                        scale,
                    ))
                    .unwrap();
            }
        }
    }
}

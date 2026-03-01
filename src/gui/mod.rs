mod colors;
mod render;

extern crate sdl2;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump, Sdl,
};

use super::ners::{Ners, StepStatus};

use std::path::Path;

// --- Config ---

struct WindowConfig {
    tile_size: usize,
    scale: u32,
    tiles_per_row: usize,
    padding: usize,
}

impl WindowConfig {
    fn new() -> Self {
        WindowConfig {
            tile_size: 8,
            scale: 2,
            tiles_per_row: 16,
            padding: 10,
        }
    }

    fn width(&self) -> u32 {
        (self.tiles_per_row * self.tile_size * self.scale as usize * 2 + self.padding) as u32
    }

    fn height(&self) -> u32 {
        (self.tiles_per_row * self.tile_size * self.scale as usize) as u32
    }

    fn right_table_offset_x(&self) -> i32 {
        (self.tiles_per_row * self.tile_size * self.scale as usize + self.padding) as i32
    }
}

#[derive(PartialEq)]
enum AppState {
    Running,
    Quit,
}

pub struct NersGui {
    core: Ners,
    ctx: Sdl,
    config: WindowConfig,
    state: AppState,
    // FIXME: Am I sure this should be here ?
    current_palette_idx: usize,
}

impl NersGui {
    pub fn new() -> Self {
        NersGui {
            core: Ners::new(),
            ctx: sdl2::init().unwrap(),

            config: WindowConfig::new(),

            state: AppState::Running,

            current_palette_idx: 0,
        }
    }

    pub fn insert_cartridge(&mut self, rom_path: impl AsRef<Path>) {
        self.core.insert_cartridge(rom_path);
    }

    pub fn run(&mut self) {
        let video_subsystem = self.ctx.video().unwrap();

        let window = video_subsystem
            .window(
                "NES Pattern Tables",
                self.config.width(),
                self.config.height(),
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = self.ctx.event_pump().unwrap();

        while self.state == AppState::Running {
            self.handle_events(&mut event_pump, &mut canvas);
            let step_status = self.core.step();
            if matches!(step_status, StepStatus::FrameReady) {
                self.render(&mut canvas);
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }
    }

    fn handle_events(&mut self, event_pump: &mut EventPump, canvas: &mut Canvas<Window>) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.state = AppState::Quit;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    self.current_palette_idx = (self.current_palette_idx + 1) % 8;
                    self.render(canvas);
                }
                _ => {}
            }
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear();

        let (left_table, right_table) = self.core.get_decoded_pattern_table();
        let palette_data = self.core.get_palettes();

        Self::draw_pattern_table(
            canvas,
            left_table,
            0,
            0,
            self.config.scale,
            &palette_data[self.current_palette_idx * 4..(self.current_palette_idx + 1) * 4],
        );

        Self::draw_pattern_table(
            canvas,
            right_table,
            self.config.right_table_offset_x(),
            0,
            self.config.scale,
            &palette_data[self.current_palette_idx * 4..(self.current_palette_idx + 1) * 4],
        );

        canvas.present();
    }
}

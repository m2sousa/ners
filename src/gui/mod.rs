mod colors;
mod config;
mod render;

extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump, Sdl};

use super::ners::{Ners, StepStatus};
use config::WindowConfig;

#[derive(PartialEq)]
enum AppState {
    Running,
    Quit,
}

pub struct NersGui {
    state: AppState,
    ctx: Sdl,

    config: WindowConfig,
    current_palette_idx: usize,
}

impl NersGui {
    pub fn new() -> Self {
        NersGui {
            ctx: sdl2::init().unwrap(),
            config: WindowConfig::new(),
            current_palette_idx: 0,
            state: AppState::Running,
        }
    }

    pub fn run(&mut self, core: &mut Ners) {
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
            self.handle_events(&mut event_pump, &mut canvas, core);

            if matches!(core.step(), StepStatus::FrameReady) {
                self.render(&mut canvas, core);
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }
    }

    fn handle_events(
        &mut self,
        event_pump: &mut EventPump,
        canvas: &mut Canvas<Window>,
        core: &Ners,
    ) {
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
                    self.render(canvas, core);
                }
                _ => {}
            }
        }
    }
}

use std::path::Path;

use crate::{bus::Bus, cpu::Cpu, loader::RomLoader, ppu::Ppu};

pub struct Ners {
    cpu: Cpu,
    ppu: Ppu,
    bus: Bus,
}

impl Ners {
    pub fn new() -> Self {
        let cpu = Cpu::new();
        let ppu = Ppu::new();
        let bus = Bus::new();

        Ners { cpu, ppu, bus }
    }

    // FIXME: Must return soem Err if file does not exists or whatever...
    pub fn insert_cartridge(&mut self, rom_path: impl AsRef<Path>) {
        let path = rom_path.as_ref();

        // FIXME: Err...
        let loader = RomLoader::load(path).unwrap();

        self.ppu.load_chr_data(&loader);

        self.bus.load_prg_data(&loader);
        self.cpu.reset(&self.bus);
    }

    pub fn run(&mut self) {
        self.cpu.run(&mut self.bus);
    }
}

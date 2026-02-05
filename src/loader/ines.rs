use super::Loader;
use super::LoaderError;

#[derive(Debug)]
pub struct INesLoader {
    data: Vec<u8>,
    prg_size: usize,
    chr_size: usize,
    prg_offset: usize,
}

impl INesLoader {
    const HEADER_SIZE: usize = 16;

    const PRG_SIZE_OFFSET: usize = 4;
    const CHR_SIZE_OFFSET: usize = 5;
    const FLAG6_OFFSET: usize = 6;
}

impl Loader for INesLoader {
    const FILE_SIGNATURE: &'static [u8] = b"NES\x1a";

    fn load_from_buffer(buf: Vec<u8>) -> Result<Self, LoaderError> {
        // PRG ROM size is encoded in 16KB units.
        let prg_size = 16384
            * *buf
                .get(Self::PRG_SIZE_OFFSET)
                .ok_or(LoaderError::IncorrectHeader)? as usize;

        // CHR ROM size is encoded in 8KB units.
        let chr_size = 8192
            * *buf
                .get(Self::CHR_SIZE_OFFSET)
                .ok_or(LoaderError::IncorrectHeader)? as usize;

        let flag6 = *buf
            .get(Self::FLAG6_OFFSET)
            .ok_or(LoaderError::IncorrectHeader)?;

        let is_trainer_present = (flag6 & Flag6Masks::TRAINER) != 0;

        let prg_offset = match !is_trainer_present {
            true => Self::HEADER_SIZE,
            false => Self::HEADER_SIZE + 512,
        };

        let loader = Self {
            data: buf,
            prg_size,
            chr_size,
            prg_offset,
        };

        Ok(loader)
    }

    fn get_prg_rom(&self) -> &[u8] {
        &self.data[self.prg_offset..self.prg_offset + self.prg_size]
    }

    fn get_chr_rom(&self) -> &[u8] {
        let chr_offset = self.prg_offset + self.prg_size;
        &self.data[chr_offset..chr_offset + self.chr_size]
    }
}

#[allow(non_snake_case)]
mod Flag6Masks {
    pub const NAMETABLE_ARRANGEMENT: u8 = 0x01;
    pub const BATTERY_BACKED: u8 = 0x02;
    pub const TRAINER: u8 = 0x04;
    pub const ALTERNATIVE_NAMETABLE_LAYOUT: u8 = 0x08;
    pub const LOWER_NYBBLE_MAPPER_NUMBER: u8 = 0xF0;
}

#[allow(non_snake_case)]
mod Flag7Masks {
    pub const VS_UNISYSTEM: u8 = 0x01;
    pub const PLAYCHOICE_10: u8 = 0x02;
    pub const NES_2_FLAG_FORMAT: u8 = 0x0c;
    pub const UPPER_NYBBLE_MAPPER_NUMBER: u8 = 0xF0;
}

mod errors;
mod ines;

use std::path::Path;

pub use errors::LoaderError;
use ines::INesLoader;

pub trait Loader {
    const FILE_SIGNATURE: &'static [u8];

    fn check_file_signature(buf: &[u8]) -> bool {
        buf.starts_with(Self::FILE_SIGNATURE)
    }

    fn load_from_buffer(buf: Vec<u8>) -> Result<Self, LoaderError>
    where
        Self: Sized;

    fn get_prg_rom(&self) -> &[u8];
    fn get_chr_rom(&self) -> &[u8];
}

#[derive(Debug)]
pub enum RomLoader {
    INes(INesLoader),
}

impl RomLoader {
    pub fn load(path: &Path) -> Result<Self, LoaderError> {
        let file_buf = std::fs::read(path).map_err(|_| LoaderError::ReadFailed)?;

        if INesLoader::check_file_signature(&file_buf) {
            INesLoader::load_from_buffer(file_buf).map(Self::INes)
        } else {
            Err(LoaderError::UnsupportedFormat)
        }
    }
}

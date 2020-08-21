use rcore_fs::dev::*;
use spin::{RwLock,Mutex};
use crate::driver::other::sdcard::SDCard;
use crate::driver::soc::spi::SPIImpl;
use k210_pac::SPI0;

pub struct Sd_card(Mutex<SDCard<SPIImpl<SPI0>>>);

impl Sd_card {
    pub fn new() -> Self {
        Sd_card(Mutex::new(crate::driver::other::sdcard::init_sdcard()))
    }
}

impl Device for Sd_card {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        let slice = self.0.read();
        let len = buf.len().min(slice.len() - offset);
        buf[..len].copy_from_slice(&slice[offset..offset + len]);
        Ok(len)
    }
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        let mut slice = self.0.write();
        let len = buf.len().min(slice.len() - offset);
        slice[offset..offset + len].copy_from_slice(&buf[..len]);
        Ok(len)
    }
    fn sync(&self) -> Result<()> {
        Ok(())
    }
}

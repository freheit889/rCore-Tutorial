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

impl BlockDevice for Sd_card {
    const BLOCK_SIZE_LOG2: u8 =9;
    fn read_at(&self, block_id: usize, buf: &mut [u8]) -> Result<()> {
        self.0.lock().read_sector(buf,block_id as u32); 
        Ok(())
    }
    fn write_at(&self, block_id: usize, buf: &[u8]) -> Result<()> {
        self.0.lock().write_sector(buf,block_id as u32);
        Ok(())
    }
    fn sync(&self) -> Result<()> {
        
        Ok(())
    }
}

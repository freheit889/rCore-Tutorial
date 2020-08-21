#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]

use core::slice;
use core::convert::TryInto;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_hal::pac::Peripherals;
use k210_shared::board::def::io;
use k210_shared::board::sdcard;
use k210_shared::soc::dmac::{dma_channel, DMACExt};
use k210_shared::soc::fpioa;
use k210_shared::soc::sysctl;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use riscv_rt::entry;


global_asm!(include_str!("link_user.S"));
/** GPIOHS GPIO number to use for controlling the SD card CS pin */
const SD_CS_GPIONUM: u8 = 7;
/** CS value passed to SPI controller, this is a dummy value as SPI0_CS3 is not mapping to anything
 * in the FPIOA */
const SD_CS: u32 = 3;

/** Connect pins to internal functions */
fn io_init() {
    fpioa::set_function(io::SPI0_SCLK, fpioa::function::SPI0_SCLK);
    fpioa::set_function(io::SPI0_MOSI, fpioa::function::SPI0_D0);
    fpioa::set_function(io::SPI0_MISO, fpioa::function::SPI0_D1);
    fpioa::set_function(io::SPI0_CS0, fpioa::function::gpiohs(SD_CS_GPIONUM));
    fpioa::set_io_pull(io::SPI0_CS0, fpioa::pull::DOWN); // GPIO output=pull down
}

fn ch(i: u8) -> char {
    if i >= 0x20 && i < 0x80 {
        i.into()
    } else {
        '.'
    }
}

fn hexdump<T: core::fmt::Write>(stdout: &mut T, buffer: &[u8], base: usize) {
    for (i, chunk) in buffer.chunks_exact(16).enumerate() {
        writeln!(stdout, "{:08x}: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            base + i * 16,
            chunk[0], chunk[1], chunk[2], chunk[3],
            chunk[4], chunk[5], chunk[6], chunk[7],
            chunk[8], chunk[9], chunk[10], chunk[11],
            chunk[12], chunk[13], chunk[14], chunk[15],
            ch(chunk[0]), ch(chunk[1]), ch(chunk[2]), ch(chunk[3]),
            ch(chunk[4]), ch(chunk[5]), ch(chunk[6]), ch(chunk[7]),
            ch(chunk[8]), ch(chunk[9]), ch(chunk[10]), ch(chunk[11]),
            ch(chunk[12]), ch(chunk[13]), ch(chunk[14]), ch(chunk[15]),
            ).unwrap();
    }
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL0, 800_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL1, 300_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL2, 45_158_400).unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);

    let serial = p
        .UARTHS
        .configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);


    io_init();

    let dmac = p.DMAC.configure();
    let spi = p.SPI0.constrain();

    let sd = sdcard::SDCard::new(spi, SD_CS, SD_CS_GPIONUM, &dmac, dma_channel::CHANNEL0);
    


    let info = sd.init().unwrap();
    let num_sectors = info.CardCapacity / 512;
    let mut sector: u32 =0;
     
    extern "C" {
        fn _user_img_start();
        fn _user_img_end();
    }
    let begin=_user_img_start as usize;
    let end=_user_img_end as usize;

    let endAlign=((end-begin)/512+1)*512;
    unsafe{
        let mut memory=slice::from_raw_parts_mut(
                begin as *mut u8,
                endAlign
            );
        for i in (end-begin)..endAlign{
            memory[i]=0;
        }
       
        writeln!(stdout,"");// fmt
        for i in 0..endAlign/512{
            sd.write_sector(&mut memory[(i)*512..(i+1)*512], sector).unwrap(); 
            sector+=1;
            if i%200==0{
                let p=(sector*100)/(endAlign/512) as u32;
                write!(stdout,"Download Sd :    [");
                for i in 0..p/2{
                    write!(stdout,"=");
                }
                for i in p/2..50{
                    write!(stdout,"-");
                }
                write!(stdout,"]{}%\r",p);
            }
        }
        writeln!(stdout, "").unwrap();

        writeln!(stdout, "succesfully write").unwrap();

    };
    loop{}
}


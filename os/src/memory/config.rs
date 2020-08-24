#![allow(dead_code)]

use lazy_static::*;
use super::address::{PhysicalAddress,VirtualAddress};

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SIZE_BITS: usize = 12;
pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8060_0000);

pub const UART_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x30000000);
pub const UART_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x40000000);
pub const SYSCTL_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x50000000);
pub const SYSCTL_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x60000000);

pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;

pub const USER_PROCESS_FRAME_QUOTA: usize = 32;
pub const KERNEL_PROCESS_FRAME_QUOTA: usize = 32;

lazy_static! {
    pub static ref KERNEL_END_ADDRESS: VirtualAddress =VirtualAddress::from(kernel_end as usize);
}

extern "C" {
    fn kernel_end();
}

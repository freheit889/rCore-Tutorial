#![no_std]
#![no_main]

extern crate alloc;
#[macro_use]
extern crate user_lib;
use alloc::string::String;
use user_lib::console::*;
use user_lib::syscall::{sys_close, sys_read};
const BUFFER_SIZE:usize=30;
const FILE:&'static str="tmp";

#[no_mangle]
pub fn main()->usize{
    println!("open file name:{}",String::from(FILE));
    let read_fd = open(FILE);
    let mut read = [0u8; BUFFER_SIZE];
    sys_read(read_fd as usize, &mut read);
    
    println!("read data={}",String::from_utf8_lossy(&read));

    sys_close(read_fd as i32);
    
    0
}


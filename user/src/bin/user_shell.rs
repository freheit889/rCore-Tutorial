#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF:u8=0x0au8;
const CR:u8=0x0du8;
const DL: u8 = 0x7fu8;

use alloc::string::String;
use user_lib::syscall::sys_exec;
use user_lib::console::getchar;

#[no_mangle]
pub fn main(){
    println!("Rust user shell");
    let mut line:String=String::new();
    
    print!(">> ");
    loop{
        let c=getchar();
        match c{
            LF|CR=>{
                println!("");
                if !line.is_empty(){
                    line.push('\0');
                    println!("");
		    println!("searching for program:{}",line);
		    sys_exec(line.as_ptr());
		    line.clear();
                }
            print!(">> ")
            }
            _=>{
                print!("{}",c as char);
                line.push(c as char);
            }
        }
    }
}


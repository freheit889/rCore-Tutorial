#![no_std]
#![no_main]
#![feature(llvm_asm)]
//#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(drain_filter)]

global_asm!(include_str!("entry.asm"));

extern crate alloc;

#[macro_use]
mod console;
mod lang_item;
mod sbi;
mod interrupt;
mod memory;
mod algorithm;
mod process;

use process::*;
use alloc::sync::Arc;

#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, sp: usize) -> ! {
    println!("Hello world #{}! sp = 0x{:x}", hartid, sp);
    interrupt::init();
    memory::init();
  //  unsafe {
    //    llvm_asm!("ebreak"::::"volatile");
    //}
    extern "C" {
        fn kernel_end();
    }
    println!("kernel_end = {:#x}", kernel_end as usize);
    println!("_kernel_end = {:#x}", (kernel_end as usize) / 4096);
 
    {
        let mut processor=PROCESSOR.lock();
        let kernel_process=Process::new_kernel().unwrap();
        for i in 1..=9usize{
            let thread=create_kernel_thread(
                    kernel_process.clone(),
                    sample_process as usize,
                    Some(&[i]),
                    );
           processor.add_thread(thread);
        }
    }
    extern "C" {
        fn __restore(context: usize);
    }
    // 获取第一个线程的 Context
    let context = PROCESSOR.lock().prepare_next_thread();
    // 启动第一个线程
    unsafe { __restore(context as usize) };
    panic!();
}
fn sample_process(id: usize) {
    println!("hello from kernel thread {}", id);
}

pub fn create_kernel_thread(
    process: Arc<Process>,
    entry_point: usize,
    arguments: Option<&[usize]>,
) -> Arc<Thread> {
    // 创建线程
    let thread = Thread::new(process, entry_point, arguments).unwrap();
    // 设置线程的返回地址为 kernel_thread_exit
    thread
        .as_ref()
        .inner()
        .context
        .as_mut()
        .unwrap()
        .set_ra(kernel_thread_exit as usize);
    thread
}

fn kernel_thread_exit() {
    // 当前线程标记为结束
    PROCESSOR.lock().current_thread().as_ref().inner().dead = true;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}



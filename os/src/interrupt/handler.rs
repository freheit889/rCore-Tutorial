use super::context::Context;
use super::timer;
use riscv::register::stvec;
use riscv::register::scause::Scause;
use riscv::register::scause::{Exception, Trap, Interrupt};
use crate::process::PROCESSOR;
use crate::kernel::syscall_handler;

global_asm!(include_str!("./interrupt.asm"));

/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize)->*mut Context {
    //panic!("Interrupted: {:?}", scause.cause());
    {
        let mut processor=PROCESSOR.lock();
        let current_thread=processor.current_thread();
        if current_thread.as_ref().inner().dead{
            println!("thread {} exit",current_thread.id);
            processor.kill_current_thread();
            return processor.prepare_next_thread();
        }
    }

    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        //Trap::Interrupt(Interrupt::SupervisorSoft) => supervisor_timer(context),
        // 其他情况，终止当前线
        Trap::Exception(Exception::UserEnvCall) => syscall_handler(context),
	Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::StoreFault)
	| Trap::Exception(Exception::InstructionFault)
	| Trap::Exception(Exception::InstructionPageFault) => page_fault(context, scause, stval),
	_ => fault("unimplemented interrupt type", scause, stval),
    }
}

fn breakpoint(context: &mut Context)->*mut Context {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
    context
}

fn supervisor_timer(context: &Context)->*mut Context {
    timer::tick();
    PROCESSOR.lock().park_current_thread(context);
    PROCESSOR.lock().prepare_next_thread()
}

fn fault(msg:&str, scause: Scause, stval: usize)->*mut Context {
    println!(
        "{:#x?} terminated: {}",
        PROCESSOR.lock().current_thread(),
        msg
    );
    println!("cause: {:?}, stval: {:x}", scause.cause(), stval);
    PROCESSOR.lock().kill_current_thread();
    PROCESSOR.lock().prepare_next_thread()

}

fn page_fault(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    static mut COUNT: usize = 0;
    /*println!("page_fault {}", unsafe {
        COUNT += 1;
        COUNT
    });*/
    let current_thread = PROCESSOR.lock().current_thread();
    let memory_set = &mut current_thread.process.inner().memory_set;
    

    match memory_set.mapping.handle_page_fault(stval) {
        Ok(_) => {
            memory_set.activate();
            context
        }
        Err(msg) => fault(msg, scause, stval),
    }
}

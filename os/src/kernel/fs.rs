//! 文件相关的内核功能

use super::*;
use core::slice::from_raw_parts_mut;
use alloc::string::String;
use crate::ROOT_INODE;
use crate::fs::inode_ext::INodeExt;
/// 从指定的文件中读取字符
///
/// 如果缓冲区暂无数据，返回 0；出现错误返回 -1
pub(super) fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    //
    let process = PROCESSOR.lock().current_thread().process.clone();
    

    if(fd==0){
        loop{
            let c=crate::console::getchar_option();
            match c{
                Some(inode)=>{
                    let buffer = unsafe { from_raw_parts_mut(buffer, 1) };
                    buffer[0]=inode as u8;
                    return SyscallResult::Proceed(1);
                }
                None=>{
                    continue;
                }
            }
        }
    }
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试读取

        if let Ok(ret) = inode.read_at(0, buffer) {
            let ret = ret as isize;
            if ret > 0 {
                return SyscallResult::Proceed(ret);
            }
            if ret == 0 {
                return SyscallResult::Park(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

/// 将字符写入指定的文件
pub(super) fn sys_write(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从进程中获取 inode
    let process = PROCESSOR.lock().current_thread().process.clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        // 尝试写入
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return SyscallResult::Proceed(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

pub(super) fn sys_open(buffer:*mut u8,size:usize)->SyscallResult{
        let fileName = unsafe {
            let buffer=from_raw_parts_mut(buffer, size);
            String::from_utf8_lossy(buffer)
        };
        //let thread=PROCESSOR.lock().current_thread();

        println!("point-2");
        ROOT_INODE.ls();
        //let file=ROOT_INODE.find(&fileName).unwrap();
        println!("point-1");
        //let thread=PROCESSOR.lock().current_thread();
        println!("point0");
        //thread.process.inner().descriptors.push(file);
        println!("point1");

        //let x:isize=(thread.process.inner().descriptors.len()-1) as isize;

        //println!("{}",x);
        SyscallResult::Proceed(1)
}

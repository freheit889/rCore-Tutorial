//! 文件系统
//!
//! 将读取第一个块设备作为根文件系统
use crate::kernel::Condvar;
use alloc::{sync::Arc, vec::Vec};
use core::any::Any;
use lazy_static::lazy_static;
use rcore_fs_sfs::SimpleFileSystem;
use spin::Mutex;

mod swap;
mod config;
pub mod inode_ext;
mod stdin;
mod stdout;
mod device;
pub use config::*;
pub use inode_ext::INodeExt;
pub use rcore_fs::{dev::block_cache::BlockCache, vfs::*};
pub use stdin::STDIN;
pub use stdout::STDOUT;
pub use swap::SwapTracker;

lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        let device={
            Arc::new(
                    device::Sd_card::new()
            )
        };
        let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
        println!("open");
        sfs.root_inode()
    };
}

/// 触发 [`static@ROOT_INODE`] 的初始化并打印根目录内容
pub fn init() {
    ROOT_INODE.ls();
    println!("mod fs initialized");
}

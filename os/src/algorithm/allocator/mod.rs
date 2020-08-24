pub mod stacked_allocator;

pub trait Allocator {
    fn new(capacity: usize) -> Self;
    fn alloc(&mut self) -> Option<usize>;
    fn dealloc(&mut self, index: usize);
}

pub use stacked_allocator::StackedAllocator;

/// 默认使用的分配器
pub type AllocatorImpl = StackedAllocator;

use super::*;
use crate::memory::*;
use crate::algorithm::{Allocator,AllocatorImpl};

const SWAP_FILE_PATH:&str="SWAP_FILE";
const SWAP_FILE_CAPACITY:usize=4096;

lazy_static!{
	static ref SWAP:Swap=Swap{
		inode:ROOT_INODE.find(SWAP_FILE_PATH).expect("cannot find swap file"),	
		allocator: Mutex::new(AllocatorImpl::new(SWAP_FILE_CAPACITY)),
	};
}

struct Swap{
	inode:Arc<dyn INode>,
	allocator:Mutex<AllocatorImpl>,
}

impl Swap{
	pub(self) fn alloc(&self)->MemoryResult<usize>{
		self.allocator.lock().alloc().ok_or("swap file full")
	}
	pub(self) fn write_page(&self,index:usize,data:&[u8;PAGE_SIZE]){
		assert!(index < SWAP_FILE_CAPACITY);
		self.inode.write_at(index*PAGE_SIZE,data).expect("failed to write swap file");
	}

	pub(self) fn read_page(&self,index:usize)->[u8;PAGE_SIZE]{
		assert!(index < SWAP_FILE_CAPACITY);
		let mut data=[0u8;PAGE_SIZE];
		self.inode.read_at(index*PAGE_SIZE,&mut data).expect("failed to read swap file");
		
		data
	}
	
	pub(self) fn dealloc(&self,index:usize){
		assert!(index < SWAP_FILE_CAPACITY);
		self.allocator.lock().dealloc(index);
	}
	
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SwapTracker(pub(super) usize);

impl SwapTracker{
	pub fn new()->MemoryResult<Self>{
		SWAP.alloc().map(Self)
	}
	
	pub fn read(&self)->[u8;PAGE_SIZE]{
		SWAP.read_page(self.0)
	}

	pub fn write(&self,data:&[u8;PAGE_SIZE]){
		SWAP.write_page(self.0,data);
	}
}

impl Drop for SwapTracker{
	fn drop(&mut self){
		SWAP.dealloc(self.0);
	}
}

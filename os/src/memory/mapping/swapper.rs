use super::*;
use crate::memory::{frame::FrameTracker,*};
use alloc::collections::VecDeque;

pub trait Swapper{
	fn new(quota:usize)->Self;
	fn full(&self)->bool;
	fn pop(&mut self)->Option<(VirtualPageNumber,FrameTracker)>;
	fn push(&mut self,vpn:VirtualPageNumber,frame:FrameTracker,entry:*mut PageTableEntry);
	fn retain(&mut self,predicate:impl Fn(&VirtualPageNumber) -> bool);
}

pub type SwapperImpl=FIFOSwapper;

pub struct FIFOSwapper{
	queue:VecDeque<(VirtualPageNumber,FrameTracker)>,
	quota:usize
}

impl Swapper for FIFOSwapper{
	fn new(quota:usize)->Self{
		Self{
			queue:VecDeque::new(),
			quota
		}
	}
	
	fn full(&self)->bool{
		self.queue.len()==self.quota
	}

	fn pop(&mut self)->Option<(VirtualPageNumber,FrameTracker)>{
		self.queue.pop_front()
	}

	fn push(&mut self,vpn:VirtualPageNumber,frame:FrameTracker,entry:*mut PageTableEntry){
		self.queue.push_back((vpn, frame));
	}
	fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        	self.queue.retain(|(vpn, _)| predicate(vpn));
    	}
}

use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::*;
use lazy_static::lazy_static;
use rcore_fs::{dev::block_cache::BlockCache, vfs::*};
use spin::Mutex;

lazy_static!{
	pub static ref FILE:Mutex<File>={
		Mutex::new(File::new())
	};
}

pub struct File{
	pub name:Vec<(String,Arc<dyn INode>)>
}

impl File{
	pub fn new()->Self{
		Self{
			name:Vec::new()
		}
	}
	pub fn push(&mut self,fileName:String,fd:Arc<dyn INode>){
		self.name.push((fileName,fd));
	}
	
	pub fn remove(&mut self,fileName:String,fd:Arc<dyn INode>){
		self.name.remove(self.name.iter().position(|(x,y)| *x==fileName).unwrap());
	}
	
	pub fn getByName(&mut self,name:String)->Arc<dyn INode>{
		self.name.iter().find(|(x,y)| *x==name).unwrap().1.clone()
	}
}

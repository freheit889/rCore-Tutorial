use riscv::register::sstatus::{self,Sstatus,SPP::*};
use core::mem::zeroed;

#[repr(C)]
#[derive(Clone,Copy,Debug)]
pub struct Context {
    pub x: [usize; 32],     // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize
}

impl Default for Context{
    fn default()->Self{
        unsafe{zeroed()}
    }
}

#[allow(unused)]
impl Context{
    pub fn sp(&self)->usize{
        self.x[2]
    }

    pub fn set_sp(&mut self,value:usize)->&mut Self{
        self.x[2]=value;
        self
    }

    pub fn ra(&self)->usize{
        self.x[1]
    }

    
    pub fn set_ra(&mut self, value: usize) -> &mut Self {
        self.x[1] = value;
        self
    }

    /// 按照函数调用规则写入参数
    ///
    /// 没有考虑一些特殊情况，例如超过 8 个参数，或 struct 空间展开
    pub fn set_arguments(&mut self, arguments: &[usize]) -> &mut Self {
        assert!(arguments.len() <= 8);
        self.x[10..(10 + arguments.len())].copy_from_slice(arguments);
        self
    }
    pub fn new(stack_top:usize,entry_point:usize,arguments:Option<&[usize]>,is_user:bool)->Self{
        let mut context=Self::default();
        context.set_sp(stack_top);
        
        if let Some(args)=arguments{
            context.set_arguments(args);
        }
        context.sepc=entry_point;
        context.sstatus=sstatus::read();
        println!("{:?}",sstatus::read()); 
        if is_user{
            context.sstatus.set_spp(User);
        }else{
            context.sstatus.set_spp(Supervisor);
        }

         context.sstatus.set_spie(true);
         context
        
    }
}



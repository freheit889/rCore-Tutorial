//! 线程调度算法
mod fifo_scheduler;
mod hrrn_scheduler;


pub trait Scheduler<ThreadType: Clone + Eq>: Default {
    /// 优先级的类型
    type Priority;
    /// 向线程池中添加一个线程
    fn add_thread(&mut self, thread: ThreadType);
    /// 获取下一个时间段应当执行的线程
    fn get_next(&mut self) -> Option<ThreadType>;
    /// 移除一个线程
    fn remove_thread(&mut self, thread: &ThreadType);
    /// 设置线程的优先级
    fn set_priority(&mut self, thread: ThreadType, priority: Self::Priority);
}

pub use fifo_scheduler::FifoScheduler;
pub use hrrn_scheduler::HrrnScheduler;

pub type SchedulerImpl<T> = HrrnScheduler<T>;

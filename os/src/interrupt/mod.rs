mod handler;
mod context;
pub mod timer;
pub use context::Context;

pub fn init() {
    handler::init();
    println!("++++ setup interrupt   ++++");
}

extern crate hex;
#[macro_use]
extern crate log;
extern crate env_logger;

mod vm;

const DEFAULTMEMSIZE: usize = 500;

fn main() {
    env_logger::init();
    let mut main_machine = vm::Virtmachine::new(DEFAULTMEMSIZE); 
    main_machine.start();
}

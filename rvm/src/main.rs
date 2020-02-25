use std::env;
use std::thread;

extern crate hex;
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate env_logger;

mod vm;
mod virtio;

const DEFAULTMEMSIZE: usize = 500;

struct virtual_machine {
    //struct to hold all of the components of the VM
    vm: vm::Virtmachine,
    virtio: virtio::Virtio,
}

impl virtual_machine {
    fn new() -> virtual_machine {
        virtual_machine {
            vm: vm::Virtmachine::new(DEFAULTMEMSIZE),
            virtio: virtio::Virtio::new(),
        }
    }

    fn run(&mut self, filename: &str) {
        self.vm.run(filename);
    }
}

fn main() {
    env_logger::init();

    //parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Not enough arguments.\n");
        println!("Usage: rvm [MODE] [FILENAME]\n");
        println!("Modes:\n\tasm: run the assembler\n\tvm: run the virtual machine");
    } else {
        let mode = &args[1];
        let filename = &args[2];

        //switch mode
        match mode.as_str() {
            "asm" => assembler_mode(filename),
            "vm" => vm_mode(filename),
            _ => println!("Unknown argument: {}", mode),
        }
    }
}

fn vm_mode(filename: &str) {
    //Run the virtual machine
    let mut vm = virtual_machine::new();
    vm.run(filename);
}

fn assembler_mode(filename: &str) {
    //Run the assembler
}

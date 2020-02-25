use std::env;

extern crate hex;
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate env_logger;

mod vm;

const DEFAULTMEMSIZE: usize = 500;

fn main() {

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
    env_logger::init();
    let mut main_machine = vm::Virtmachine::new(DEFAULTMEMSIZE); 
    main_machine.start(filename);
}

fn assembler_mode(filename: &str) {
    //Run the assembler
}

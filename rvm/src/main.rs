use std::{
    env,
    thread,
    sync::mpsc,
    time::Duration,
};

extern crate hex;
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate env_logger;

mod vm;
mod virtio;
mod assembler;

const DEFAULTMEMSIZE: usize = 500;

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

    //create channel for I/O signals
    let (iotx, iorx) = mpsc::channel();
    //create VM and get some channel stuff from it
    let (mut vm, vmbuf, vmctrl) = vm::Virtmachine::new(DEFAULTMEMSIZE);
    //create virtual io
    let mut virtio = virtio::Virtio::new(iorx, vmbuf);

    //load binary into VM memory
    vm.load(filename);

    //run VM thread
    let vmthread = thread::spawn(move ||{
        vm.run();
    });

    //run thread to check control signals
    thread::spawn(move ||{
        loop {
            thread::sleep(Duration::from_millis(1));
            let control = match vmctrl.try_recv() {
                Ok(v) => v,
                Err(_v) => 0,
            };
            if control == 1 {
                iotx.send(1).unwrap();
                break;
            }
        } 
    });

    //run virtual I/O thread
    thread::spawn(move ||{
        virtio.run();
    });

    //wait for VM to exit
    vmthread.join().unwrap();
}

fn assembler_mode(filename: &str) {
    //Run the assembler
    let mut asm = assembler::Assembler::new(filename);
}

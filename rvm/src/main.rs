use std::env;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

extern crate hex;
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate env_logger;

mod vm;
mod virtio;

const DEFAULTMEMSIZE: usize = 500;

/*struct virtual_machine {
    //struct to hold all of the components of the VM
    vm: vm::Virtmachine,
    vmbuf: std::sync::mpsc::Receiver<char>,
    virtio: virtio::Virtio,
    ioctrl: std::sync::mpsc::Sender<u32>,
    vmctrl: std::sync::mpsc::Receiver<u32>,
}

impl virtual_machine {
    fn new() -> virtual_machine {
        let (iotx, iorx) = mpsc::channel();
        let (vm, vmbuf, vmctrl) = vm::Virtmachine::new(DEFAULTMEMSIZE);
        virtual_machine {
            vm: vm,
            vmbuf: vmbuf,
            virtio: virtio::Virtio::new(iorx, vmbuf),
            ioctrl: iotx,
            vmctrl: vmctrl,
        }
    }

    fn run(&mut self, filename: &'static str) {
        let vmthread = thread::spawn(move ||{
            self.vm.run(filename);
        });
        let controlthread = thread::spawn(move ||{
            while true {
                thread::sleep(Duration::from_millis(1));
                let control = match self.vmctrl.try_recv() {
                    Ok(v) => v,
                    Err(v) => 0,
                };
                if control == 1 {
                    self.ioctrl.send(1);
                    break;
                }
            } 
        });
        let iothread = thread::spawn(move ||{
            self.virtio.run();
        });
    }
}*/

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
    /*let mut vm = virtual_machine::new();
    vm.run(filename);*/
    let fname = filename;

    let (iotx, iorx) = mpsc::channel();
    let (mut vm, vmbuf, vmctrl) = vm::Virtmachine::new(DEFAULTMEMSIZE);
    let mut virtio = virtio::Virtio::new(iorx, vmbuf);

    let vmthread = thread::spawn(move ||{
        vm.run(filename);
    });
    thread::spawn(move ||{
        while true {
            thread::sleep(Duration::from_millis(1));
            let control = match vmctrl.try_recv() {
                Ok(v) => v,
                Err(v) => 0,
            };
            if control == 1 {
                iotx.send(1);
                break;
            }
        } 
    });
    thread::spawn(move ||{
        virtio.run();
    });

    vmthread.join().unwrap();
}

fn assembler_mode(filename: &str) {
    //Run the assembler
}

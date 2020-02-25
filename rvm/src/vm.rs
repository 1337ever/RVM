use log::debug;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use byteorder::{ReadBytesExt, NativeEndian};

pub struct Virtmachine {
    mem: Vec<u32>,
    ip: usize, //instruction pointer
    ef: u32, //continue flag, execution stops if it equals 1
    zf: u32, //zero flag
    of: u32, //overflow flag
}

impl Virtmachine {

    pub fn new(memsize: usize) -> Virtmachine {
        //construct a new virtual machine
        debug!("Creating new Virtmachine with size {}", memsize);
        Virtmachine {
            mem: vec![0u32; memsize],
            ip: 0usize,
            ef: 0u32,
            zf: 0u32,
            of: 0u32,
        }
    }

    fn u32_to_u8_array(x: u32) -> [u8; 4] {
        let b1: u8 = ((x >> 24) & 0xff) as u8;
        let b2: u8 = ((x >> 16) & 0xff) as u8;
        let b3: u8 = ((x >> 8) & 0xff) as u8;
        let b4: u8 = (x & 0xff) as u8;
        return [b1, b2, b3, b4]
    }

    fn u8_array_to_u32_le(array: &[u8; 4]) -> u32 {
        ((array[0] as u32) << 0) +
        ((array[1] as u32) << 8) +
        ((array[2] as u32) << 16) +
        ((array[3] as u32) << 24)
    }

    pub fn start(&mut self, filename: &str) {
        //load a binary into mem and start the VM
        debug!("Starting virtual machine with size {}", self.mem.len());
        debug!("Attempting to load file {}", filename);

        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(reason) => panic!("failed to open {}: {}", display, reason.description()),
            Ok(file) => file,
        };

        let mut buf_reader = BufReader::new(file);
        //read file into mem by u32 int
        buf_reader.read_u32_into::<NativeEndian>(&mut self.mem[..]);
        
        while self.ef != 1 {
            self.cycle();
            self.ip += 1; //increment instruction pointer
        }

    }

    fn cycle(&mut self) {
        //complete one cycle of execution
        let opcode = self.mem[self.ip];
        debug!("Exec: [{:x}]:{:x}", self.ip, opcode);
        let oparray = Virtmachine::u32_to_u8_array(opcode);
        match oparray[0] {
            0xff => self.op_eof(),
            0x01 => self.op_mov(oparray),
            0x02 => self.op_str(oparray),
            _ => debug!("Unrecognized opcode"),
        }
    }

    fn set_mem(&mut self, index: usize, value: u32) {
        //set an area of memory to a value
        self.mem[index] = value;
        debug!("Memset:\n [{:x}]:{:x}", index, value);
    }
    fn get_mem(&self, index: usize) -> u32 {
        self.mem[index]
    }

    fn add(&mut self, index: usize, value: u32) {
        let mut result = self.get_mem(index);
        match result.checked_add(value) {
            Some(res) => {
                self.set_mem(index, result);
            }
            None => {
                debug!("Overflow when adding {:x} to [{:x}]({:x})", value, index, self.get_mem(index));
                self.of = 1;
            }
        }
    }
    fn sub(&mut self, index: usize, value: u32) {
        let result = self.get_mem(index) - value;
        self.set_mem(index, result);
    }
    
    //Operations:
    fn op_eof(&mut self) {
        debug!("\tEOF reached.");
        self.ef = 1;
    }
    fn op_mov(&mut self, oparry: [u8; 4]) {
        //move value from one address into another
        
    }
    fn op_str(&mut self, oparray: [u8; 4]) {
        //store a literal constant into an address
        let store = Virtmachine::u8_array_to_u32_le(&[0x00, 0x00, oparray[2], oparray[3]]);
        self.set_mem(oparray[1] as usize, store);
    }
}

use log::debug;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, NativeEndian, BigEndian};

pub struct Virtmachine {
    mem: Vec<u32>,
    ip: u32, //instruction pointer
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
            ip: 0u32,
            ef: 0u32,
            zf: 0u32,
            of: 0u32,
        }
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
        buf_reader.read_u32_into::<BigEndian>(&mut self.mem[..]);
        
        while self.ef != 1 {
            self.cycle();
            self.ip += 1; //increment instruction pointer
        }

    }

    fn cycle(&mut self) {
        //complete one cycle of execution
        let opcode = self.mem[self.ip as usize];
        debug!("Exec: [{:#010x}]:{:#010x}", self.ip, opcode);
        //let oparray = Virtmachine::u32_to_u8_array(opcode);
        let mut oparray = Vec::new();
        oparray.write_u32::<BigEndian>(opcode).unwrap();
        match oparray[0] {
            0xff => self.op_eof(),
            0x01 => self.op_mov(oparray),
            0x02 => self.op_str(oparray),
            0x03 => self.op_add(oparray),
            0x04 => self.op_sub(oparray),
            _ => debug!("Unrecognized opcode"),
        }
    }

    fn set_flag(&mut self, name: &str, value: u32) {
        //set flag by its name as a &str
        debug!("Setting flag {} to {:#010x}", name, value);
        match name {
            "ip" => self.ip = value,
            "ef" => self.ef = value,
            "zf" => self.zf = value,
            "of" => self.of = value,
            _ => debug!("Unknown flag"),
        }
    }

    fn set_mem(&mut self, index: usize, value: u32) {
        //set an area of memory to a value
        self.mem[index] = value;
        debug!("Memset:\n\t[{:#010x}]:{:#010x}", index, value);
    }
    fn get_mem(&self, index: usize) -> u32 {
        self.mem[index]
    }

    fn add(&mut self, index: usize, value: u32) {
        let mut result = self.get_mem(index);
        match result.checked_add(value) {
            Some(res) => {
                self.set_mem(index, result + value);
            }
            None => {
                debug!("Overflow when adding {:#010x} to [{:#010x}]({:#010x})", value, index, self.get_mem(index));
                self.set_flag("of", 1);
            }
        }
    }
    fn sub(&mut self, index: usize, value: u32) {
        let mut result = self.get_mem(index);
        match result.checked_sub(value) {
            Some(res) => {
                self.set_mem(index, result - value);
                if result - value == 0 {
                    self.set_flag("zf", 1);
                }
            }
            None => {
                debug!("Overflow when subtracting {:#010x} from [{:#010x}]({:#010x})", value, index, self.get_mem(index));
                self.set_flag("of", 1);
            }
        }
    }
    
    //Operations:
    fn op_eof(&mut self) {
        debug!("\tEOF reached.");
        self.ef = 1;
    }
    fn op_mov(&mut self, oparry: Vec<u8>) {
        //move value from one address into another
        
    }
    fn op_str(&mut self, oparray: Vec<u8>) {
        //store a literal constant into an address
        let mut storebuf = Cursor::new(vec![0x00, 0x00, oparray[2], oparray[3]]);
        let store = storebuf.read_u32::<BigEndian>().unwrap();
        self.set_mem(oparray[1] as usize, store);
    }
    fn op_add(&mut self, oparray: Vec<u8>) {
        let index = oparray[1] as usize;
        let mut valbuf = Cursor::new(vec![0x00, 0x00, oparray[2], oparray[3]]);
        let value = valbuf.read_u32::<BigEndian>().unwrap();
        self.add(index, value);
    }
    fn op_sub(&mut self, oparray: Vec<u8>) {
        let index = oparray[1] as usize;
        let mut valbuf = Cursor::new(vec![0x00, 0x00, oparray[2], oparray[3]]);
        let value = valbuf.read_u32::<BigEndian>().unwrap();
        self.sub(index, value);
    }
}

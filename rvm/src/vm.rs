//the core of the vm. runs binary files.

use std::{
    fs::File,
    io::{BufReader, Cursor},
    path::Path,
    sync::mpsc::{self, Receiver},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use log::debug;

pub struct Virtmachine {
    //Struct for the "CPU" of the VM
    mem: Vec<u32>,
    ip: u32, //instruction pointer
    ef: u32, //continue flag, execution stops if it equals 1
    zf: u32, //zero flag
    of: u32, //overflow flag
    charsender: std::sync::mpsc::Sender<char>,
    status_sender: std::sync::mpsc::Sender<u32>,
}

impl Virtmachine {
    pub fn new(memsize: usize) -> (Virtmachine, Receiver<char>, Receiver<u32>) {
        //construct a new virtual machine
        debug!("Creating new Virtmachine with size {}", memsize);
        let (ctx, crx) = mpsc::channel();
        let (stx, srx) = mpsc::channel();
        (
            Virtmachine {
                mem: vec![0u32; memsize],
                ip: 0u32,
                ef: 0u32,
                zf: 0u32,
                of: 0u32,
                charsender: ctx,
                status_sender: stx,
            },
            crx,
            srx,
        )
    }

    pub fn load(&mut self, filename: &str) {
        //load a binary into memory
        debug!("Attempting to load file {}", filename);

        let path = Path::new(filename);
        let display = path.display();

        let file = match File::open(&path) {
            Err(reason) => panic!("failed to open {}: {}", display, reason.to_string()),
            Ok(file) => file,
        };

        let mut buf_reader = BufReader::new(file);
        //read file into mem by u32 int
        buf_reader.read_u32_into::<BigEndian>(&mut self.mem[..]);
    }

    pub fn run(&mut self) {
        //start the VM
        debug!("Starting virtual machine with size {}", self.mem.len());

        let mut cycles = 0;
        while self.ef != 1 {
            cycles += 1;
            self.cycle();
            self.ip += 1; //increment instruction pointer
        }
    }

    pub fn print_mem(&self, start: usize, end: usize) {
        //print a segment of memory for inspection
        debug!("Memory slice {:#010x} to {:#010x}", start, end);
        let mut i = start;
        while i <= end {
            debug!("[{:#010x}:{:#010x}]", i, self.mem[i]);
            i += 1;
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
            0x03 => self.op_adi(oparray),
            0x04 => self.op_sui(oparray),
            0x05 => self.op_jmp(oparray),
            0x06 => self.op_jz(oparray),
            0x07 => self.op_cmp(oparray),
            0x08 => self.op_prn(oparray),
            0x09 => self.op_mul(oparray),
            0x0A => self.op_div(oparray),
            0x0B => self.op_mop(oparray),
            _ => debug!("Unrecognized operation"),
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

    fn compose_u32(array: Vec<u8>) -> u32 {
        let mut buf = Cursor::new(vec![array[0], array[1], array[2], array[3]]);
        buf.read_u32::<BigEndian>().unwrap()
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
        let result = self.get_mem(index);
        match result.checked_add(value) {
            Some(_) => {
                self.set_mem(index, result + value);
            }
            None => {
                debug!(
                    "Overflow when adding {:#010x} to [{:#010x}]({:#010x})",
                    value,
                    index,
                    self.get_mem(index)
                );
                self.set_flag("of", 1);
            }
        }
    }
    fn sub(&mut self, index: usize, value: u32) {
        let result = self.get_mem(index);
        match result.checked_sub(value) {
            Some(_) => {
                self.set_mem(index, result - value);
                if result - value == 0 {
                    self.set_flag("zf", 1);
                }
            }
            None => {
                debug!(
                    "Overflow when subtracting {:#010x} from [{:#010x}]({:#010x})",
                    value,
                    index,
                    self.get_mem(index)
                );
                self.set_flag("of", 1);
            }
        }
    }

    //Operations:
    fn op_eof(&mut self) {
        self.set_flag("ef", 1);
        debug!("\tEOF reached.");
        self.status_sender.send(1).unwrap();
    }
    fn op_mov(&mut self, oparray: Vec<u8>) {
        //move value from one address into another
        let dest = oparray[1] as usize;
        let srcvalue = self.get_mem(oparray[3] as usize);
        self.set_mem(dest, srcvalue);
    }
    fn op_mop(&mut self, oparray: Vec<u8>) {
        //move value from one address into another, using src as a pointer
        let dest = oparray[1] as usize;
        let src = self.get_mem(self.get_mem(oparray[3] as usize) as usize);
        self.set_mem(dest, src);
    }
    fn op_str(&mut self, oparray: Vec<u8>) {
        //store a literal constant into an address
        let index = oparray[1] as usize;
        let store = Virtmachine::compose_u32(vec![0x00, 0x00, oparray[2], oparray[3]]);
        self.set_mem(index, store);
    }
    fn op_adi(&mut self, oparray: Vec<u8>) {
        //add an inline value to an address
        let index = oparray[1] as usize;
        let value = Virtmachine::compose_u32(vec![0x00, 0x00, oparray[2], oparray[3]]);
        self.add(index, value);
    }
    fn op_sui(&mut self, oparray: Vec<u8>) {
        //subtract an inline value from an address
        let index = oparray[1] as usize;
        let value = Virtmachine::compose_u32(vec![0x00, 0x00, oparray[2], oparray[3]]);
        self.sub(index, value);
    }
    fn op_jmp(&mut self, oparray: Vec<u8>) {
        //jump to an address
        //the ip flag is set to the target address minus one because after the cycle finishes, the
        //ip will be incremented automatically
        let value = Virtmachine::compose_u32(vec![0x00, 0x00, 0x00, oparray[1] - 1]);
        debug!("Jumping to {:#010x}", value + 1);
        self.set_flag("ip", value);
    }
    fn op_jz(&mut self, oparray: Vec<u8>) {
        //jump to an address if the result of the previous operation was zero
        if self.zf == 1 {
            self.op_jmp(oparray);
        }
    }
    fn op_cmp(&mut self, oparray: Vec<u8>) {
        //compare two numbers by subtraction and set zero flag
        //without actually modifying any location in memory
        self.set_flag("zf", 0);
        let x = self.get_mem(oparray[1] as usize);
        let y = self.get_mem(oparray[3] as usize);
        match x.checked_sub(y) {
            None => self.set_flag("zf", 1),
            _ => (),
        }
    }
    fn op_prn(&mut self, oparray: Vec<u8>) {
        let fullval = self.get_mem(oparray[1] as usize);
        let schar = fullval.to_le_bytes()[0];
        self.charsender.send(schar as char).unwrap();
    }
    fn op_mul(&mut self, oparray: Vec<u8>) {
        //multiply the value in one address by the value in another address
        let dest = oparray[1] as usize;
        let destval = self.get_mem(oparray[1] as usize);
        let src = self.get_mem(oparray[3] as usize);
        self.set_mem(dest, destval * src);
    }
    fn op_div(&mut self, oparray: Vec<u8>) {
        //divide the value in one address by the value in another address
        let dest = oparray[1] as usize;
        let destval = self.get_mem(oparray[1] as usize);
        let src = self.get_mem(oparray[3] as usize);
        self.set_mem(dest, destval / src);
    }
}

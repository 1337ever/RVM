use log::debug;

pub struct Virtmachine {
    mem: Vec<u32>,
    ip: u32, //instruction pointer
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
            zf: 0u32,
            of: 0u32,
        }
    }

    pub fn start(&mut self) {
        
    }

    fn set_mem(&mut self, index: usize, value: u32) {
        //set an area of memory to a value
        self.mem[index] = value;
        debug!("Memset:\n [{:x}]:{:x}", index, value);
    }
    fn get_mem(&self, index: usize) -> u32 {
        self.mem[index]
    }
    
    //Operations:
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
}

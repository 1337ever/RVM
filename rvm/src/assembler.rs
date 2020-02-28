//module to take a file and output a binary that the vm can then run

use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
        Cursor,
    },
    path::Path,
    error::Error,
};
use byteorder::{
    WriteBytesExt, 
    ReadBytesExt, 
    NativeEndian, 
    BigEndian, 
    LittleEndian
};
use regex::Regex;

//definitions for operations
const RAWDEFS: &str = r#"
    {
        "items": [
            {
                "name": "eof",
                "code": 255,
                "args": 0
            },
            {
                "name": "mov",
                "code": 1,
                "args": 2
            },
            {
                "name": "str",
                "code": 2,
                "args": 2
            },
            {
                "name": "adi",
                "code": 3,
                "args": 2
            },
            {
                "name": "sui",
                "code": 4,
                "args": 2
            },
            {
                "name": "jmp",
                "code": 5,
                "args": 1
            },
            {
                "name": "jz",
                "code": 6,
                "args": 1
            },
            {
                "name": "cmp",
                "code": 7,
                "args": 2
            },
            {
                "name": "prn",
                "code": 8,
                "args": 1
            }
        ]
    }"#;

fn compose_u32(array: Vec<u8>) -> u32 {
    let mut buf = Cursor::new(vec![array[0], array[1], array[2], array[3]]);
    buf.read_u32::<BigEndian>().unwrap()
}

//struct to hold some data about different operations
#[derive(Deserialize)]
struct Operation {
    name: String, //a string for the name of the operation
    code: u8, //the "number" in machine code that this op corresponds to
    args: usize, //the number of arguments this operation takes
}

#[derive(Deserialize)]
struct Obj {
    items: Vec<Operation>,
}


struct Element {
    op: u8,
    args: Vec<u8>,
    composed: u32
}

impl Element {
    fn new(defs: &Vec<Operation>, ops: Vec<&str>) -> Element {
        let mut top: u8 = 0; 
        let mut nops = vec![0; 4];

        let mut tops = ops.to_vec();
        tops.remove(0);

        for def in defs {
            if def.name == ops[0] {
                top = def.code;
            }
        }
        if tops.len() > 0 {
            nops = tops.iter().map(|x| 
                x.parse::<u8>().unwrap()
            ).collect();
        }

        let tcomposed = compose_u32(vec![top, nops[0], 0, nops[1]]);
        
        Element {
            op: top,
            args: nops,
            composed: tcomposed
        }
    }
}


pub struct Assembler {
    stringfile: Vec<String>, //a vector of strings from the input file
    binary: Vec<u32>, //a vector of u32 for the output binary
    definitions: Vec<Operation>, //a vector containing definitions of operations
}

impl Assembler {
    pub fn new(filename: &str) -> Assembler {
        //load a file and create the Assembler
        debug!("Creating new Assembler from file {}", filename);
        let file = File::open(filename).expect("Assembly file not found.");
        let buf = BufReader::new(file);
        let lines = buf.lines()
            .map(|l| l.expect("Failed to read line."))
            .collect();
        debug!("File contents:\n{:#?}", lines);

        let defs = serde_json::from_str::<Obj>(RAWDEFS).unwrap();

        Assembler {
            stringfile: lines,
            binary: Vec::new(),
            definitions: defs.items,
        }
    }

    pub fn gen_binary(&mut self) {
        let mut i = 0i8;
        for line in &self.stringfile {
            let result = self.assemble(line);
            match result {
                Some(v) => self.binary.insert(i as usize, v),
                None => i -= 1,
            }            
            i += 1;
        }

        //turn vec of u32 into slice of u8
        let mut bytesvec = Vec::new();
        for v in &self.binary {
            let mut tempvec = Vec::new();
            tempvec.write_u32::<LittleEndian>(*v).unwrap();
            bytesvec.append(&mut tempvec);
        }

        let path = Path::new("a.out");
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(reason) => panic!("Failed to create file {}: {}", display, reason.description()),
            Ok(file) => file,
        };
        match file.write_all(bytesvec.as_slice()) {
            Err(reason) => panic!("Failed to write to {}: {}", display, reason.description()),
            Ok(_) => println!("Wrote binary to {}", display),
        }
    }

    //assemble one line
    fn assemble(&self, input: &String) -> Option<u32> {
        debug!("Assembling \"{}\"", input);
        let re = Regex::new(r";.*").unwrap(); 
        let t2 = re.replace_all(input, "");
        let mut temp: Vec<&str> = t2.split(" ").collect();
        temp.remove_item(&"");

        debug!("{:?}", temp);

        if temp.len() == 0 {
            return None
        }

        let elem = Element::new(&self.definitions, temp);


        Some(elem.composed)
    }
}

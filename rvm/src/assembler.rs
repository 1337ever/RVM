//module to take a file and output a binary that the vm can then run

use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
};

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

#[derive(Deserialize)]
struct Operation {
    name: String, //a string for the name of the operation
    code: u8, //the "number" in machine code that this op corresponds to
    args: u8, //the number of arguments this operation takes
}

#[derive(Deserialize)]
struct Obj {
    items: Vec<Operation>,
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
}

//module to take a file and output a binary that the vm can then run

use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
    path::Path,
};

pub struct Assembler {
    stringfile: Vec<String>, //a vector of strings from the input file
    binary: Vec<u32>, //a vector of u32 for the output binary
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

        Assembler {
            stringfile: lines,
            binary: Vec::new(),
        }
    }
}

use std::fs::File;
use std::io::prelude::*;
pub struct Rom {
    pub program: [u8; 3584],
    pub length: usize
}

impl Rom {
    pub fn read_rom(path: &str) -> Rom {
        let mut file = match File::open(path){
            Ok(file) => file,
            Err(e) => panic!("{e}")
        };

        let mut instance = Rom {
            program: [0; 3584],
            length: 0
        };

        match file.read(&mut instance.program) {
            Ok(n) => instance.length = n,
            Err(e) => panic!("{e}")
        };

        instance
    }
}
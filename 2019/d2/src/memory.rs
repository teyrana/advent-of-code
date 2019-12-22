use std::fs;

#[derive(Default)]
pub struct Memory {
    data: Vec<u32>,
    address: usize,
}

impl Memory {
    pub fn at( &self ) -> usize {
        self.address
    }

    pub fn len( &self) -> usize {
        self.data.len()
    }

    pub fn peek( &self ) -> u32 {
        self.data[self.address]
    }

    pub fn read( &self, address: usize ) -> u32 {
        self.data[address]
    }

    pub fn seek( &mut self, address: usize ){
        self.address = address;
    }

    pub fn write( &mut self, address: usize, value: u32 ) {
        self.data[address] = value;
    }
}

impl Memory {
    pub fn new() -> Memory {
        Default::default()
    }

    pub fn from_input_file() -> Memory {
        let mut mem = Memory::new();

        let input_filename = String::from("input.csv");
        // println!("  Reading input: {}", input_filename); 

        let read_result = fs::read_to_string(input_filename);
        if read_result.is_err() {
            return mem;
        }
        let contents: String = read_result.unwrap();
        
        let tokens: Vec<&str> = contents.split(",").collect();

        // cheat--this is the exact length of my input.
        mem.data.resize(160, 99);

        for (index, str_value) in tokens.iter().enumerate() {
            let int_value: u32 = str_value.trim().parse::<u32>().unwrap();
            mem.write(index as usize, int_value);
        }

        mem
    }
}
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_input_file( buffer: String) -> Result< Vec<u16>, &'static str> {
    
    let tokens: Vec<&str> = buffer.split(",").collect();

    let mut numbers: Vec<u16> = Vec::new()
    // cheat--this is the exact length of my input.
	numbers.reserve_exact(160);
	
    for (index, str_value) in tokens.iter().enumerate() {
        let int_value: u16 = str_value.trim().parse::<u16>().unwrap();
        numbers[index] = int_value;
    }

    Ok(numbers)
}

fn main() -> io::Result<()> {
    println!("Starting Day 2:");
    
    let input_filename = String::from("input.csv");
    // println!("  Reading input: {}", input_filename); 

    let mut inf = File::open(input_filename)?;
    let mut buffer: String = String::new();
    inf.read_to_string(&mut buffer)?;

    let mut input = read_input_file( buffer ).unwrap();

    input[1] = 12;
    input[2] = 2;
    
    // put actual code here
    println!("....Start processing:");
    println!("....???");
    println!("....Profit!");

    println!(">>>> position 0 = {}", input[0]);
    
    Ok(())
}

// use std::io;
// use std::io::{Error, ErrorKind};

mod opcode;
mod memory;

fn main() -> Result<(), &'static str> {

    println!("Starting Day 2:");

    println!("....Loading input:");
    let mut mem = memory::Memory::from_input_file();

    // if read_result.is_err() {
    //     return Err(-1)
    // }


    println!("....Loading state:");
    mem.write(1,12);
    mem.write(2,2);
    
    // put actual code here
    println!("....Start processing:");
    println!(".......input size: {}", mem.len() );

    loop {
		let result: Result<(), &str> = opcode::process(&mut mem);
		if result.is_ok() {
            // no-op
			println!(".... @[{}] = {}", mem.at(), mem.peek());
		}else{
            let opcode = mem.peek();
            if 99 == opcode {
                println!("<< Done.");
            }else{
                println!("!! Could not process opcode at: [{}] = {}", mem.at(), mem.peek());
                return Err("Error processing opcode!!")
            }
            break;
        }
	}
	
    println!(">>>>  @[0] = {}", mem.read(0));
    
    Ok(())
}

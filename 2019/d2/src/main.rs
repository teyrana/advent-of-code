use std::env;

mod opcode;
mod memory;

fn run_program( mut mem: &mut memory::Memory ) -> Result<u32, &'static str> {
    loop {
        let result: Result<(), &str> = opcode::process(&mut mem);
        if result.is_ok() {
            // no-op
            // println!(".... @[{}] = {}", mem.at(), mem.peek());
            print!(".");
        }else{
            let opcode = mem.peek();
            if 99 == opcode {
                let output: u32 = mem.read(0);
                return Ok(output)
            }else{
                println!("!! Could not process opcode at: [{}] = {}", mem.at(), mem.peek());
                return Err("Error processing opcode!!")
            }
        }
    }
    
}

fn main() -> Result<(), &'static str> {

    println!("Starting Day 2:");

    let args: Vec<String> = env::args().collect();

    println!("... processing {} CLA", args.len());

    let mut search: bool = false;
    let mut noun : u32 = 12;
    let mut verb : u32 = 2;

    if 2 < args.len() {
        println!(">> Using CL Args:      '{}', '{}'", args[1], args[2]);

        let noun_result = args[1].trim_matches(',').parse::<u32>();
        let verb_result = args[2].trim_matches(',').parse::<u32>();
        if noun_result.is_ok() && verb_result.is_ok() {
            noun = noun_result.unwrap();
            verb = verb_result.unwrap();
        }else{
            return Err("!! could not parse command line arguments! Aborting! !!")
        }
        println!(">> Using CL Args:      {}, {}", noun, verb);
    }else if 1 < args.len() {
        if "default" == args[1] {
            println!(">> Using default args: {}, {} ", noun, verb);
        }else if "search" == args[1] {
            println!(">> Using 'search' mode.");
            search = true;
        }
    }else{ 
        println!(">> Using default args: {}, {} ", noun, verb);
    }

    println!("....Loading Image:");
    let initial_memory = memory::Memory::from_input_file();
    println!(".......input size: {}", initial_memory.len() );
    
    if search {
        println!("....Start search:");
        let target_output: u32 = 19690720;
        println!(".......target output: {}", target_output );

        for noun in 0..100 {
            for verb in 0..100 {
        // for noun in 0..3 {
        //     for verb in 0..3 {
                let mut mem = initial_memory.clone();
                
                mem.write(1, noun);
                mem.write(2, verb);
                
                let program_result = run_program( &mut mem);

                if program_result.is_ok(){
                    let program_output = program_result.unwrap();
                    if target_output == program_output {
                        println!(" >> found target output with input: {} => {}", noun*100 + verb, program_output );
                        return Ok(())
                    }else{
                        println!(" >> program( {} ) => {} ", noun*100 + verb, program_output );
                    }
                }
            }
        }
        Err("An exhaustive search failed to find the desired target?!")

    }else{ 
        let mut mem = initial_memory.clone();
        
        mem.write(1, noun);
        mem.write(2, verb);

        println!("... running program:");
        let program_result = run_program( &mut mem);
        println!("\\");

        if program_result.is_ok() {
            println!(">>  Program input: {} => output: {}", noun*100 + verb, program_result.unwrap() );
            return Ok(())
        }else{
            println!(">>  Error running program, with input: {}", noun*100 + verb );
            Err( ">>  Error running program." )
        }
    }
}

use crate::memory;

pub fn process( mut buffer: &mut memory::Memory ) -> Result< (), &'static str> {
	let opcode = buffer.peek();
	
	if 1 == opcode {
		return process_add( &mut buffer );
	} else if 2 == opcode {
		return process_mult( &mut buffer );
	} else if 99 == opcode {
		return Err("Done!")
	}
	
	Err("Unrecognized opcode!!")
}

fn process_add( mem: &mut memory::Memory ) -> Result< (), &'static str> {
	let address = mem.at();
	let param_1_index: usize = mem.read(address + 1) as usize;
	let param_2_index: usize = mem.read(address + 2) as usize;
	let param_3_index: usize = mem.read(address + 3) as usize;

	// println!("        <1> @ {}-{}", index, index+3);

	mem.write(param_3_index, mem.read(param_1_index) + mem.read(param_2_index));
	mem.seek(address + 4);

	Ok(())
}

fn process_mult( mem: &mut memory::Memory ) -> Result< (), &'static str> {
	let address = mem.at();
	let param_1_index: usize = mem.read(address + 1) as usize;
	let param_2_index: usize = mem.read(address + 2) as usize;
	let param_3_index: usize = mem.read(address + 3) as usize;
	
	// println!("        <2> @ {}-{}", index, index+3);

	mem.write(param_3_index, mem.read(param_1_index) * mem.read(param_2_index));
	mem.seek(address + 4);

	Ok(())
}

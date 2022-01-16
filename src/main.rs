mod registers;
mod opcodes;
mod condition_flags;
mod program_counter;

use registers::Registers;
use condition_flags::ConditionFlags;
use program_counter::ProgramCounter;
use opcodes::OpCode;
use std::env;
use std::process::exit;

// constants
const MEMORY_LOCATION: i32 = 65536 ;
const NUM_REGISTERS : i32 = Registers::RCount as i32;

#[allow(unused_variables)]
fn main() {

    // Reading the command line arguments
    let args : Vec<String> = env::args().collect();
    if args.len() < 2{
        //Show usage string
        println!("lc3 [image-file] ...");
        exit(2);
    }

    for i in args.iter(){

        if !read_image(i){
            println!("Failed to load the image : {}", args[i]);
            exit(1);
        }
    }

    //Memory
    let memory : [u16; MEMORY_LOCATION as usize] = [0; MEMORY_LOCATION as usize];

    //Registers
    let registers : [u16; NUM_REGISTERS as usize] = [];
    println!("The last register is {:?}", Registers::RCount as i32);

    registers[Registers::RCond as u16] = ConditionFlags::FlZro as u16;
    registers[Registers::RPc as u16] = ProgramCounter::PcStart as u16;

    let mut running = 1;
    while running{

        // Fetch
        let mut instr = mem_read(registers[Registers::RPc as u16]);
        let mut op = instr >> 12;
        let mut opcode_value = OpCode::from(op);

        match opcode_value {
            OpCode::OpAdd => println!("Add"),
            OpCode::OpAnd => println!("And"),
            OpCode::OpNot => println!("Not"),
            OpCode::OpBr => println!("Break"),
            OpCode::OpJmp => println!("Jump"),
            OpCode::OpJsr => println!("Jsr"),
            OpCode::OpLd => println!("Ld"),
            OpCode::OpLdi => println!("Ldi"),
            OpCode::OpLdr => println!("Ldr"),
            OpCode::OpLea => println!("Lea"),
            OpCode::OpSt => println!("St"),
            OpCode::OpSti => println!("Sti"),
            OpCode::OpStr => println!("Str"),
            OpCode::OpTrap => println!("Trap"),
            OpCode::OpRes => println!("Res"),
            OpCode::OpRti => println!("Rti"),
            _ => println!("Bad Opcode"),


        }
    }


}

fn mem_read(value : u16) -> u16{
    0
}

#[cfg(test)]
mod tests{
    use crate::Registers;
    use super::*;

    #[test]
    fn anything(){}
}


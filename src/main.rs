mod registers;
mod opcodes;
mod condition_flags;
mod program_counter;
mod instructions_impl;

use registers::Registers;
use condition_flags::ConditionFlags;
use program_counter::ProgramCounter;
use opcodes::OpCode;
use std::env;
use std::process::exit;

// constants
const MEMORY_LOCATION: i32 = 65536 ;
const NUM_REGISTERS : i32 = Registers::RCount as i32 - 1;

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

        if !read_image(&i){
            println!("Failed to load the image : {}", i);
            exit(1);
        }
    }

    //Memory
    let memory : [u16; MEMORY_LOCATION as usize] = [0; MEMORY_LOCATION as usize];

    //Registers
    let mut registers : [u16; NUM_REGISTERS as usize] = [0; NUM_REGISTERS as usize];
    println!("The last register is {:?}", Registers::RCount as i32);

    registers[Registers::RCond as usize] = ConditionFlags::FlZro as u16;
    registers[Registers::RPc as usize] = ProgramCounter::PcStart as u16;

    let mut running = true;
    while running{

        // Fetch
        let  instr = mem_read(registers[Registers::RPc as usize]);
        let  op = instr >> 12;
        let  opcode_value = OpCode::from(op);

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

fn mem_read(mut value :  u16) -> u16{
    value += 1 ;
    value
}

fn read_image(file : &String) -> bool{
    false
}


fn sign_extend(mut x: u16, bit_count : i32) -> u16{

    if (x >> (bit_count - 1)) & 1 > 0 {
        x |= (0xFFF << bit_count);
    }
    x
}

fn update_flags(r : u16,  reg: &mut [u16]){

    let val = reg[r as usize] >> 15;
    match reg[r as usize]{
        0 =>  {reg[Registers::RCond as usize] = ConditionFlags::FlZro as u16 },
        val => reg[Registers::RCond as usize] = ConditionFlags::FlNeg as u16, // a 1 in the leftmost bit indicates negative
        _ => reg[Registers::RCond as usize] = ConditionFlags::FlPos as u16,
    }
}

#[cfg(test)]
mod tests{
    use crate::Registers;
    use super::*;

    #[test]
    fn anything(){}
}


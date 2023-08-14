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


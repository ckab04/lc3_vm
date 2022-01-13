mod registers;
mod opcodes;
mod condition_flags;

use registers::Registers;

// constants
const MEMORY_LOCATION: i32 = 65536 ;
const NUM_REGISTERS : i32 = Registers::RCount as i32;

#[allow(unused_variables)]
fn main() {
    println!("Hello, world!");
    //Memory
    let memory : [u16; MEMORY_LOCATION as usize] = [0; MEMORY_LOCATION as usize];

    //Registers
    let registers : [u16; NUM_REGISTERS as usize];
    println!("The last register is {:?}", Registers::RCount as i32);



}

#[cfg(test)]
mod tests{
    use crate::Registers;
    use super::*;

    #[test]
    fn anything(){}
}


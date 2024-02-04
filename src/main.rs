use crate::hardware::Registers;
use crate::hardware::Registers::*;

mod hardware;

const MEMORY_MAX: u32 = 1 << 16;
const NUM_REGISTERS: u16 = RCOUNT as u16;


fn main() {

    let memory:  [u16; MEMORY_MAX as usize]= [0; MEMORY_MAX as usize];
    //let LENGTH_REGISTER_STORAGE: usize = register_storage as usize;
    let registers: [u16; NUM_REGISTERS as usize] = [0; NUM_REGISTERS as usize];
    println!("Hello rust");
}






use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::hardware::Registers;
use crate::hardware::Registers::*;

mod hardware;
mod instructions;
mod trap_codes;

const MEMORY_MAX: u32 = 1 << 16;
const NUM_REGISTERS: u16 = RCOUNT as u16;


fn main() {

    let memory:  [u16; MEMORY_MAX as usize]= [0; MEMORY_MAX as usize];
    //let LENGTH_REGISTER_STORAGE: usize = register_storage as usize;
    let registers: [u16; NUM_REGISTERS as usize] = [0; NUM_REGISTERS as usize];
    println!("Hello rust");
    println!("#{:06x}", trap_codes::TRAP::TrapGet as i32);
}


fn read_image_file(file_path: PathBuf){

    let mut content = String::new();
    let mut file = File::open(file_path).unwrap();
    let origin = file.read_to_string(&mut content).unwrap();

    let origin = swap16(origin as u16);
    /* we know the maximum file size so we only need one fread */
}

fn swap16(x: u16)-> u16{
    (x << 8 ) | (x >> 8)
}





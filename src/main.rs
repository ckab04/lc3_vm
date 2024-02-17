use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::hardware::Registers;
use crate::hardware::Registers::*;

mod hardware;
mod instructions;
mod trap_codes;
mod registers;

const MEMORY_MAX: u32 = 1 << 16;
const NUM_REGISTERS: u16 = RCOUNT as u16;


fn main() {

    let memory:  [u16; MEMORY_MAX as usize]= [0; MEMORY_MAX as usize];
    //let LENGTH_REGISTER_STORAGE: usize = register_storage as usize;
    let registers: [u16; NUM_REGISTERS as usize] = [0; NUM_REGISTERS as usize];
    println!("Hello rust");
    println!("#{:06x}", trap_codes::TRAP::TrapGet as i32);
}


fn read_image_file(file : File, memory: u32){

    let mut content = String::new();
    //let mut file = File::open(&file_path).unwrap();
    let f_read = file.clone();
    let origin = f_read.read(&mut content).unwrap();

    let origin = swap16(origin as u16);
    /* we know the maximum file size so we only need one fread */

    let max_read = MEMORY_MAX - origin as u32;
    let mut p = (memory + origin as u32) as u16;
    //let file = File::open(file_path);

    //let mut file =  File::open(&file_path).unwrap();
    let mut file =  file.clone();
    let mut buffer = String::new();
    let mut read_size = file.read_to_string(&mut buffer).unwrap();
    while read_size > 0 {
        p = swap16(p);
        p +=1;
    }


}

fn swap16(x: u16)-> u16{
    (x << 8 ) | (x >> 8)
}

fn read_image(image_length: &PathBuf){
    let  _file = File::open(&image_length).unwrap();
    //read_image_file()
}



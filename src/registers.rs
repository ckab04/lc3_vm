use std::io;
use crate::{MEMORY_MAX, NUM_REGISTERS};


#[derive(Debug)]
pub enum MemoryMappedRegisters{
    MrKBSR = 0xFE00,
    MrKBDR = 0xFE02,
}

// Memory access

fn mem_write(address: u16, val: u16, memory: &mut [u16; MEMORY_MAX as usize]){
    memory[address as usize] = val;

}

fn mem_read(address: u16, memory: &mut [u16; MEMORY_MAX as usize]) -> u16{
    let v1 = MemoryMappedRegisters::MrKBSR as u16;
    let v2 = MemoryMappedRegisters::MrKBDR as u16;

    if address == v1{
        if check_key(){
            memory[v1 as usize] = 1 << 15;
            let ch = get_char();
            let val = u32::from(ch) as u16;
            memory[v2 as usize] = val; 
        }else{
            memory[v1 as usize] = 0;
        }        
    }
    memory[address]
}

fn check_key() -> bool {
    todo!()
}

fn get_char()-> char{
    let mut input = String::new();
    let _value = io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let first_char = input.chars()
        .nth(0).unwrap();

    first_char
}
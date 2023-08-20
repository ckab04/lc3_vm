use std::io::{Read, stdin, stdout, Write};
use crate::components::operations::update_flags;
use crate::components::registers::Registers::R_R0;
use crate::components::trap_codes::TRAP_CODES::*;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TRAP_CODES{
    TRAP_GETC ,  /* get character from keyboard, not echoed onto the terminal */
    TRAP_OUT ,   /* output a character */
    TRAP_PUTS ,  /* output a word string */
    TRAP_IN ,    /* get character from keyboard, echoed onto the terminal */
    TRAP_PUTSP , /* output a byte string */
    TRAP_HALT    /* halt the program */

}

impl From<TRAP_CODES> for u16{
    fn from(value: TRAP_CODES) -> Self {
       match value{
          TRAP_GETC => 0x20,
           TRAP_OUT => 0x21,
           TRAP_PUTS => 0x22,
           TRAP_IN => 0x23,
           TRAP_PUTSP => 0x24,
           TRAP_HALT => 0x25
       }
    }
}

impl From<u16> for TRAP_CODES{
    fn from(value: u16) -> Self {
        match value{
            0x20 => TRAP_GETC,
            0x21 => TRAP_OUT,
            0x22 => TRAP_PUTS,
            0x23 => TRAP_IN,
            0x24 => TRAP_PUTSP,
            0x25 => TRAP_HALT,
            _ => TRAP_HALT,
        }
    }
}

// TRAP Getc
pub (crate) fn op_getc(mut reg: &mut Vec<u16>){
    // read a single ascii char
    let rr0 = u16::from(R_R0) as usize;
    //let _my_char = char::from_u32(input).unwrap();

    reg[rr0] = get_char() as u16;
    update_flags(rr0 as u16, reg);
}


pub(crate) fn op_outc(reg: &Vec<u16>){
    put_char(reg);
}

pub fn op_in(mut reg: &mut Vec<u16>){
    println!("Enter a character : ");
    let char_c = get_char();
    print!("{}", char::from_u32(char_c).unwrap());
    let rr0 = u16::from(R_R0) as usize;
    let value_rr0 = *reg.get(rr0).unwrap();
    reg[rr0] = value_rr0;
    update_flags(rr0 as u16, reg);

}

fn put_char(reg: &Vec<u16>){
    let rr0 = u16::from(R_R0) as usize;
    let value_in_rr0 = *reg.get(rr0).unwrap() as u32;
    let rr0_in_char = char::from_u32(value_in_rr0).unwrap();
    print!("{}", rr0_in_char);
    stdout().flush().ok();
}

// Get char from stdin
fn get_char() -> u32{
    let mut input = stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u32).unwrap() ;

    input
}






























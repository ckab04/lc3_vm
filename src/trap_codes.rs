use std::io;
use std::io::Write;
use crate::hardware::Registers;
use crate::hardware::Registers::{RPC, RR0, RR7};
use crate::instructions::update_flags;
use crate::NUM_REGISTERS;
use crate::trap_codes::TRAP::{TrapHalt, TrapIn, TrapOut, TrapPuts, TrapPutsp};

#[derive(Debug)]
pub enum TRAP{
    TrapGet = 0x20, /* get character from keyboard, not echoed onto the terminal */
    TrapOut = 0x21, /* output a character */
    TrapPuts = 0x22, /* output a word string */
    TrapIn = 0x23, /* get character from keyboard, echoed onto the terminal */
    TrapPutsp = 0x24, /* output a byte string */
    TrapHalt = 0x25,  /* halt the program */

}

pub fn execute_trap_code(
    trap: TRAP,
    instr: u16,
    reg: &mut [u16; NUM_REGISTERS as usize]
){

    let rr7 = Registers::from(RR7);
    let rpc = Registers::from(RPC);

    let switch_value = instr >> 0xFF;
    let trap_get = TRAP::TrapGet as u16;
    let trap_out = TrapOut as u16;
    let trap_puts = TrapPuts as u16;
    let trap_in = TrapIn as u16;
    let trap_putsp = TrapPutsp as u16;
    let trap_halt = TrapHalt as u16;
    match switch_value{
        trap_get => println!("TrapGet"),
        trap_out => println!("TrapOUt"),
        trap_puts => println!("TrapPuts"),
        trap_in => println!("TrapIn"),
        trap_putsp => println!("TrapPuts"),
        trap_halt => println!("Trap Halt"),
    }
}

fn trap_getc(reg: &mut [u16; NUM_REGISTERS as usize]){
    let mut input = String::new();
    let _value = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    //let first_char = input.chars().nth(0).unwrap();
    let first_char = input.bytes()
        .nth(0)
        .map(|result| result as u16).unwrap();

    let rr0 = Registers::from(RR0) as u16;
    reg[rr0] = first_char;
    update_flags(rr0);
}

fn trap_out_f(reg: &mut [u16; NUM_REGISTERS as usize]){
   let rr0 = Registers::from(RR0) as u16;
    let reg_rr0 = reg[rr0];
    let rr0_char = char::from(reg_rr0);
    print!("{}", rr0_char);
    io::stdout().flush().unwrap();
}

fn trap_in_f(reg: &mut [u16; NUM_REGISTERS as usize]){

}
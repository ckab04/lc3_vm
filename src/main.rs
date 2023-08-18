use crate::components::instruction_set::Opcodes;
use crate::components::operations::op_add;
use crate::components::registers::{get_registers, Registers};
mod components;

fn main() {
    println!("Hello there !!!");

    let val = Registers::R_R7;
    //let i = Registers::from(val);
    let s: u16 = val.into();
    //println!("Value obtained {:?}", i);
    println!("Value of s = {}", s);

}

fn run(){

    let mut reg = get_registers();

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    let rcond = u16::from(Registers::R_COND) as usize;
    let val = components::condition_flags::ConditionFlags::FL_ZRO as u16;
    reg[rcond] = val;


    // Set the PC to starting position
    // 0x3000 is the default
    let pc_start = 0x3000;
    let rpc = u16::from(Registers::R_PC) as usize;
    reg[rpc] = pc_start;
    let running = 1;

    while running == 1{
        let m = reg[rpc];

        // FETCH
        let instr = components::memory::mem_read(m);
        let mut op = instr >> 12;


       if op == u16::from(Opcodes::OP_ADD){
            let v = op_add(instr,  &mut reg);
        }

    }
}





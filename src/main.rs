use crate::components::registers::{get_registers, Registers};
mod components;

fn main() {
    println!("Hello there !!!");

    let val = Registers::R_R7;
    //let i = Registers::from(val);
    let s: i32 = val.into();
    //println!("Value obtained {:?}", i);
    println!("Value of s = {}", s);

}


fn run(){

    let mut reg = get_registers();

    /* since exactly one condition flag should be set at any given time, set the Z flag */
    reg[Registers::R_COND] = components::condition_flags::ConditionFlags::FL_ZRO;


    // Set the PC to starting position
    // 0x3000 is the default
    let pc_start = 0x3000;

    reg[Registers::R_PC] = pc_start;
    let running = 1;

    while running == 1{

        // FETCH
        let instr = components::memory::mem_read(reg[Registers::R_PC].into());
        let mut op = instr >> 12;


        match op{

        }

    }
}



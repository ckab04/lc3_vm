use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::components::instruction_set::Opcodes;
use crate::components::operations::*;
use crate::components::registers::{get_registers, Registers};
use crate::components::registers::Registers::{R_PC, R_R7};
use crate::components::trap_codes::{op_getc, op_halt, op_in, op_outc, op_puts, op_putsp, TRAP_CODES};
use crate::components::trap_codes::TRAP_CODES::{TRAP_GETC, TRAP_HALT, TRAP_IN, TRAP_OUT, TRAP_PUTS, TRAP_PUTSP};

mod components;

fn main() {
    println!("Hello there !!!");

    let val = R_R7;
    //let i = Registers::from(val);
    let s: u16 = val.into();


    let a = u16::from(val);
    let b: Registers = a.into();
    //println!("Value obtained {:?}", i);
    println!("Value of s = {}", s);
    println!("Value of a = {}", a);
    println!("Value of b = {:?}", b);

}


// Read image file
fn read_image_file(file : &File, MEMORY_MAX : u16){
    /* the origin tells us where in memory to place the image */

    let origin : u16;
    let memory = 0;
    let f = File::open("myFile.txt").unwrap();
    let reader = BufReader::new(f);
    let mut content = reader.lines();
    let v = content.next().unwrap().unwrap();
    origin = swap16(v.parse().unwrap());

    let max_read = MEMORY_MAX - origin;
    let mut p = memory + origin;

    for val in content{
        p = swap16(p);
        p +=1;
    }

}

fn swap16(x: u16) -> u16{
    x << 8 | x >> 8
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

        // Using if statement because match does not support function call


       if op == u16::from(Opcodes::OP_ADD){
             op_add(instr,  &mut reg);
        }
        else if op == u16::from(Opcodes::OP_AND){
                op_and(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_NOT){
            op_not(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_BR){
            op_branch(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_JMP){
            op_jump(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_JSR){
            op_jump_register(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_LD){
            op_load(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_LDI){
            op_load_indirect(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_LDR){
            op_load_register(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_LEA){
            op_load_effective_address(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_ST){
            op_store(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_STI){
            op_store_indirect(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_STR){
            op_store_register(instr, &mut reg);
        }
        else if op == u16::from(Opcodes::OP_TRAP){
            let rr7 = u16::from(R_R7) as usize;
            let rpc = u16::from(R_PC) as usize;
            reg[rr7] = reg[rpc];
            let trap_code = instr & 0xFF;
            let trap_c = TRAP_CODES::from(trap_code);

            match trap_c {
                TRAP_GETC => { op_getc(&mut reg)},
                TRAP_OUT => {  op_outc(&mut reg)},
                TRAP_PUTS => { op_puts(&mut reg)},
                TRAP_IN => { op_in(&mut reg)},
                TRAP_PUTSP => { op_putsp(&mut reg)},
                TRAP_HALT => { op_halt(running) }
            }

        }

    }
}





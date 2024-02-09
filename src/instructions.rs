use crate::hardware::Registers;
use crate::hardware::Registers::*;
use crate::NUM_REGISTERS;

pub fn add(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    /* destination register (DR) */
    let r0 = ((instr >> 9) & 0x7 ) as usize;

    /* first operand (SR1) */
    let r1 = ((instr >> 6 ) & 0x7 ) as usize;

    let imm_flag = (instr >> 5) & 0x1;
    //let val_from_r0 = Registers::from(RR0) as usize ;
    //let val_from_r1 = Registers::from(RR1) as usize ;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        reg[r0] = reg[r1] + imm5;
    }else {

        let r2 = instr & 0x7;
        reg[r0] = reg[r1] + r2;

    }
    update_flags(r0.try_into().unwrap());
}

fn update_flags(p0: u16) {
    todo!()
}

fn sign_extend(p0: u16, p1: i32) -> u16 {
    todo!()
}
pub fn and_instr(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    let r0 = ((instr >> 9) & 0x7) as usize;
    let r1 = ((instr >> 6 ) & 0x7) as usize;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1{
        let  imm5 = sign_extend(instr & 0x1F, 5);
        reg[r0] = reg[r1] & imm5;
    }else {
        let r2 = (instr & 0x7) as usize;
        reg[r0] = reg[r1] & reg[r2];
    }

    update_flags(r0.try_into().unwrap());
}

pub fn not_instr(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    let r0 = ((instr >> 9) & 0x7) as usize;
    let r1 = ((instr >> 6 ) & 0x7) as usize;

    reg[r0] = !reg[r1];
    update_flags(r0.try_into().unwrap());
}

pub fn branch_instr(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){

    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;
    let rcond_val = Registers::from(RCOND) as usize;
    let rpc_val = Registers::from(RPC) as usize;
    if (cond_flag & reg[rcond_val]) == 1 {
        let mut val = reg[rpc_val];
        val += pc_offset;
        reg[rpc_val] = val;
    }
}

pub fn jump_instr(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    let r1 = (instr >> 6) & 0x7;
    let rpc_val = Registers::from(RPC) as usize;
    reg[rpc_val] = reg[r1 as usize];
}

pub fn jump_register(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    let long_flag = (instr >> 11);
    let rr7 = Registers::from(RR7) as usize;
    let rpc = Registers::from(RPC) as usize;
    reg[rr7] = reg[rpc];
    if long_flag == 1{
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        reg[rpc] += long_pc_offset; // JSR
    }else{
        let r1 = (instr >> 6) & 0x7;
        reg[rpc] = reg[r1];
    }
}



pub fn load(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let rpc = Registers::from(RPC) as usize;
    reg[r0 as usize] = mem_read(reg[rpc] + pc_offset);
    update_flags(r0);
}

fn mem_read(p0: u16) -> u16 {
    todo!()
}























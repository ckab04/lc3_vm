use crate::hardware::Registers;
use crate::hardware::Registers::*;
use crate::NUM_REGISTERS;

pub fn add(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    /* destination register (DR) */
    let r0 = (instr >> 9) & 0x7usize;

    /* first operand (SR1) */
    let r1 = (instr >> 6 ) & 0x7usize;

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
    update_flags(r0);
}

fn update_flags(p0: u16) {
    todo!()
}

fn sign_extend(p0: u16, p1: i32) -> u16 {
    todo!()
}
pub fn and_instr(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6 ) & 0x7usize;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1{
        let  imm5 = sign_extend(instr & 0x1F, 5);
        reg[r0] = reg[r1] & imm5;
    }else {
        let r2 = instr & 0x7;
        reg[r0] = reg[r1] & reg[r2];
    }

    update_flags(r0);
}

pub fn not_instr(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6 ) & 0x7usize;

    reg[r0] = !reg[r1];
    update_flags(r0);
}

pub fn branch_instr(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){

    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;
    if cond_flag & reg[Registers::from(RCOND)] {
        let mut val = reg[Registers::from(RPC)];
        val += pc_offset;
        reg[Registers::from(RPC)] = val;
    }
}

pub fn jump_instr(instr: u16, reg: &mut [u16; NUM_REGISTERS as usize]){
    let r1 = (instr >> 6) & 0x7;
    reg[Registers::from(RPC)] = reg[r1];
}

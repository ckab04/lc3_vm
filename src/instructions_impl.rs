use crate::{sign_extend, update_flags};

fn add_instr(instr: u16, reg: &mut [u16]){

//destination register
    let r0 : u16 = (instr >> 9) & 0x7;

    //First operand
    let r1 : u16 = (instr >> 6) & 0x7;

    //Check whether we are in immediate mode
    let imm_flag  = (instr >>5) & 0x1;

    if imm_flag > 0 {
        let imm5 = sign_extend((instr & 0x1F), 5);
        reg[r0 as usize] = reg[r1 as usize]  + imm5;
    }else {

        let r2 = instr & 0x7;
        reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
    }

    update_flags(r0, reg);
}
use crate::components::condition_flags::ConditionFlags;
use crate::components::registers::Registers;


// ADD instruction
pub fn op_add(instr : u16, mut reg: &mut Vec<u16>){

    // Destination Register (DR)
    let r0 = ((instr >> 9) & 0x7);
    let r0 = usize::try_from(r0).unwrap();

    // first operand  (SR1)
    let _r1 = ((instr >> 6) & 0x7 );
    let r1 = usize::try_from(r0).unwrap();

    // whether we are in immediate mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1{
        let imm5 = sign_extend(instr & 0x1F, 5);
        reg[r0] = reg[r1] + imm5;

    }
    else{
        let r2 = (instr & 0x7) as usize;
        reg[r0] = reg[r1] + reg[r2];
    }

    update_flags(r0 as u16, reg);

}

fn op_and(instr : u16, mut reg: &mut Vec<u16>){
    // Destination Register (DR)
    let r0 = ((instr >> 9) & 0x7) as usize;
    //let r0 = usize::try_from(r0).unwrap();

    // first operand  (SR1)
    let r1 = ((instr >> 6) & 0x7 ) as usize;
    //let r1 = usize::try_from(r0).unwrap();

    // whether we are in immediate mode
    let imm_flag = (instr >> 5) & 0x1;
    if imm_flag > 0{
      let imm5 = sign_extend(instr >> 0x1F, 5);
        reg[r0] = reg[r1] & imm5;
    }
    else {
        let r2 = (instr & 0x7) as usize;
        reg[r0] = reg[r1] + reg[r2];
    }
    update_flags(r0 as u16, reg);
}

fn op_not(instr : u16, mut reg: &mut Vec<u16>){
   let r0 = ((instr >> 9) & 0x7) as usize;
    let r1 = ((instr >> 6) & 0x7) as usize;
    reg[r0] = !reg[r1];
    update_flags(r0 as u16, reg);

}

fn sign_extend(mut x: u16, bit_count : u16) -> u16{
    let sign = x >> (bit_count - 1) & 1;

    if  sign > 0 {

        x |= (0xFFF >> bit_count);
    }
    x
}

fn update_flags(r : u16, mut reg: &mut Vec<u16>){

    let rcond = u16::from(Registers::R_COND) as usize;
    let cond_flag_zero =i16::from(ConditionFlags::FL_ZRO);
    let cond_flag_neg = i16::from(ConditionFlags::FL_NEG);
    let cond_flag_pos = i16::from(ConditionFlags::FL_POS);

    if reg[r as usize] == 0{
        reg[rcond] = cond_flag_zero as u16;
    }
    else if reg[r as usize] >> 15 < 0 {
        reg[rcond] = cond_flag_neg as u16;
    }
    else{
        reg[rcond] = cond_flag_pos as u16;
    }

}
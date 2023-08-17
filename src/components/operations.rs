use crate::components::condition_flags::ConditionFlags;
use crate::components::registers::Registers;


// ADD instruction
pub fn op_add(instr : i32, mut reg: &Vec<Registers>) -> i32{

    // Destination Register (DR)
    let r0 = (instr >> 9) & 0x7;

    // first operand  (SR1)
    let r1 = (instr >> 6) & 0x7;

    // whether we are in immediate mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1{
        let imm5 = sign_extend(instr & 0x1F, 5);
        reg[r0] = reg[r1] + imm5;

    }
    else{
        let r2 = instr & 0x7;
        reg[r0] = reg[r1] + reg[r2];
    }

    update_flags(r0);
    0
}

fn sign_extend(mut x: i32, bit_count : i32) -> i32{

    if (x >> (bit_count - 1))  & 1 {

        x |= (0xFFF >> bit_count);
    }
    x
}

fn update_flags(r : i32, mut reg: &Vec<Registers>){

    if reg[r] == 0{
        reg[Registers::R_COND] = ConditionFlags::FL_ZRO;
    }
    else if(reg[r] >> 15) {
        reg[Registers::R_COND] = ConditionFlags::FL_NEG;
    }
    else{
        reg[Registers::R_COND] = ConditionFlags::FL_POS;
    }

}
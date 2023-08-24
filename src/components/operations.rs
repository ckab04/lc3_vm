use crate::components::common_op::register_0_pc_offset;
use crate::components::condition_flags::ConditionFlags;
use crate::components::memory::{mem_read, mem_write};
use crate::components::registers::Registers;


// ADD instruction
#[allow(unused_parens)]
pub fn op_add(instr : u16,  reg: &mut Vec<u16>){

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

// ADD
#[allow(arithmetic_overflow)]
pub fn op_and(instr : u16,  reg: &mut Vec<u16>){
    // Destination Register (DR)
    let r0 = ((instr >> 9) & 0x7) as usize;
    //let r0 = usize::try_from(r0).unwrap();

    // first operand  (SR1)
    let r1 = ((instr >> 6) & 0x7 ) as usize;
    //let r1 = usize::try_from(r0).unwrap();

    // whether we are in immediate mode
    let imm_flag = (instr >> 5) & 0x1;
    if imm_flag > 0{
        let v = instr >> 0x1F;
      let imm5 = sign_extend(v, 5);
        reg[r0] = reg[r1] & imm5;
    }
    else {
        let r2 = (instr & 0x7) as usize;
        reg[r0] = reg[r1] + reg[r2];
    }
    update_flags(r0 as u16, reg);
}

// NOT
pub(crate) fn op_not(instr : u16,  reg: &mut Vec<u16>){
   let r0 = ((instr >> 9) & 0x7) as usize;
    let r1 = ((instr >> 6) & 0x7) as usize;
    reg[r0] = !reg[r1];
    update_flags(r0 as u16, reg);

}

// BRANCH INSTRUCTION

pub (crate) fn op_branch(instr : u16,  reg: &mut Vec<u16>){

    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let cond_flag = (instr >> 9) & 0x7;
    let val_rcond = u16::from(Registers::R_COND);

    if cond_flag &  reg.get(val_rcond as usize).unwrap()> 0{
        let index = u16::from(Registers::R_PC);
        let a = reg.get(index as usize).unwrap() + pc_offset;
        let rpc = u16::from(Registers::R_PC) as usize;
        reg[rpc] = a;
    }

}

// JUMP
pub (crate) fn op_jump(instr : u16,  reg: &mut Vec<u16>){
    // Also handles RET
    let r1 = ((instr >> 6) & 0x7) as usize;
    let rpc = u16::from(Registers::R_PC) as usize;
    reg[rpc] = *reg.get(r1).unwrap();
}


// JUMP REGISTER
pub (crate) fn op_jump_register(instr : u16,  reg: &mut Vec<u16>){

    let long_flag = (instr >> 11) & 1;
    let rr7 = u16::from(Registers::R_R7) as usize;
    let rpc = u16::from(Registers::R_PC) as usize;
    reg[rr7] = reg[rpc];

    if long_flag > 0{

        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        let rpc_content = *reg.get(rpc).unwrap() + long_pc_offset; // JSR
        reg[rpc] = rpc_content;
    }
    else{
        let r1 = ((instr >> 6) & 0x7) as usize;
        reg[rpc] = reg[r1]; // JSRR
    }
}

// LOAD
pub (crate) fn op_load(instr : u16,  reg: &mut Vec<u16>){

    let r0 = ((instr >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let rpc = u16::from(Registers::R_PC) as usize;
    reg[r0] = mem_read(*reg.get(rpc).unwrap() + pc_offset);
    update_flags(r0 as u16, reg);
}


// LDI load indirect
pub (crate) fn op_load_indirect(instr : u16,  reg: &mut Vec<u16>){
    //Destination register (DR)

    let r0 = ((instr >> 9) & 0x7) as usize;
    // PC OFFSET 9

    let pc_offset = sign_extend(instr & 0x1FF, 9);
    /* add pc_offset to the current PC, look at that memory location to get the final address */
    let rpc = u16::from(Registers::R_PC) as usize;
    let value = *reg.get(rpc).unwrap() + pc_offset;
    reg[r0] = mem_read(value);
    update_flags(r0 as u16, reg);
}

// Load Register LDR
pub(crate)  fn op_load_register(instr : u16,  reg: &mut Vec<u16>){
    let r0 = ((instr >> 9) & 0x7) as usize;
    let r1 = ((instr >> 6) & 0x7) as usize;
    let offset = sign_extend(instr & 0x3F, 6);
    let val_r1 = *reg.get(r1).unwrap();
    reg[r0] = mem_read(val_r1 + offset);
    update_flags(r0 as u16, reg);
}

// Load effective address
pub (crate) fn op_load_effective_address(instr : u16,  reg: &mut Vec<u16>){
    let r0 = ((instr >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let rpc = u16::from(Registers::R_PC) as usize;
    let val_rpc = *reg.get(rpc).unwrap() + pc_offset;
    reg[r0] = val_rpc + pc_offset;
    update_flags(r0 as u16, reg);
}



// Store
pub (crate) fn op_store(instr : u16, reg: &mut Vec<u16>){
    let r0 = ((instr >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let rpc = u16::from(Registers::R_PC) as usize;
    let val_rpc = *reg.get(rpc).unwrap() + pc_offset;
    let value_in_r0 = *reg.get(r0 as usize).unwrap();
    mem_write(val_rpc, value_in_r0);
}

// Store indirect

pub (crate) fn op_store_indirect(instr : u16, reg: &mut Vec<u16>){

    let (r0, sum_rpc_pc_offset) = register_0_pc_offset(instr, reg);
    let value_in_r0 = *reg.get(r0 as usize).unwrap();
    mem_write(mem_read(sum_rpc_pc_offset), value_in_r0);
}

// Store register
pub (crate) fn op_store_register(instr : u16, reg: &mut Vec<u16>){
    let r0 = ((instr >> 9) & 0x7) as usize;
    let r1 = ((instr >> 6) & 0x7) as usize;

    let offset = sign_extend(instr & 0x3F, 6);
    let value_in_r1 = *reg.get(r1).unwrap();
    let value_in_r0 = *reg.get(r0).unwrap();
    mem_write(value_in_r1 + offset, value_in_r0);

}

// TRAP ROUTINES

// pub (crate) fn op_trap(instr : u16, reg: &mut Vec<u16>){
//
// }

// TRAPS

pub (crate) fn sign_extend(mut x: u16, bit_count : u16) -> u16{
    let sign = x >> (bit_count - 1) & 1;

    if  sign > 0 {

        x |= (0xFFF >> bit_count);
    }
    x
}

pub (crate) fn update_flags(r : u16, reg: &mut Vec<u16>){

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
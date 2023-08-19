use crate::components::operations::sign_extend;
use crate::components::registers::Registers;

pub fn register_0_pc_offset(instr : u16, mut reg: &mut Vec<u16>) -> (u16, u16){  // Values returned are R_R0 and R_PC
    let r0 = ((instr >> 9) & 0x7) as usize;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let rpc = u16::from(Registers::R_PC) as usize;
    let val_rpc = *reg.get(rpc).unwrap() + pc_offset;
    (r0 as u16, val_rpc)
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Registers{
    R_R0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC, /* program counter */
    R_COND,
    R_COUNT
}

impl From<Registers> for i32{
    fn from(value: Registers) -> Self {
       match value{
           Registers::R_R0 => 0,
           Registers::R_R1 => 1,
           Registers::R_R2 => 2,
           Registers::R_R3 => 3,
           Registers::R_R4 => 4,
           Registers::R_R5 => 5,
           Registers::R_R6 => 6,
           Registers::R_R7 => 7,
           Registers::R_PC => 8,
           Registers::R_COND => 9,
           Registers::R_COUNT => 10,
       }
    }
}

pub fn get_registers()->Vec<Registers>{

    vec![]
}
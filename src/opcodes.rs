use std::intrinsics::unreachable;

pub enum OpCode{
    OpBr = 0,  // Branch
    OpAdd = 1, // add
    OpLd = 2,  // Load
    OpSt = 3,  // Store
    OpJsr = 4, // Jump register
    OpAnd = 5, //  Bitwise and
    OpLdr = 6, // Load Register
    OpStr = 7, // Store register
    OpRti = 8, //   Unused
    OpNot = 9, //  Bitwise not
    OpLdi = 10, // Load indirect
    OpSti = 11, // Store indirect
    OpJmp = 12, // Jump
    OpRes = 13, // Reserved(Unused)
    OpLea = 14, // Load effective address
    OpTrap = 15, // Execute trap
}

impl From<u16> for OpCode{
    fn from(val : u16) -> Self {
        match val{

            0 => OpCode::OpBr,
            1=> OpCode::OpAdd,
            2 => OpCode::OpLd,
            3 => OpCode::OpSt,
            4 => OpCode::OpJsr ,
            5 => OpCode::OpAnd ,
            6 => OpCode::OpLdr ,
            7 => OpCode::OpStr ,
            8 => OpCode::OpRti ,
            9 => OpCode::OpNot ,
            10 => OpCode::OpLdi ,
            11 => OpCode::OpSti ,
            12 => OpCode::OpJmp ,
            13 => OpCode::OpRes ,
            14 => OpCode::OpLea ,
            15 => OpCode::OpTrap ,
            _ => unreachable!()
        }
    }
}
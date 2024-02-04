use crate::hardware::Registers::*;

#[repr(C)]
#[derive(Debug)]
pub enum Registers{
    RR0 = 0,
    RR1,
    RR2,
    RR3,
    RR4,
    RR5,
    RR6,
    RR7,
    RPC, /* program counter */
    RCOND,
    RCOUNT
}

impl From<Registers> for u16{
    fn from(value: Registers) -> Self {
       match value{
           RR0 => 0,
           RR1=> 1,
           RR2=> 2,
           RR3=> 3,
           RR4=> 4,
           RR5=> 5,
           RR6=> 6,
           RR7=> 7,
           RPC=> 8, /* program counter */
           RCOND=> 9,
           RCOUNT=> 10,
       }
    }
}

impl From<u16> for Registers{
    fn from(value: u16) -> Self {

        match value{
            0 => RR0,
            1 => RR1,
            2 => RR2,
            3 => RR3,
            4 => RR4,
            5 => RR5,
            6 => RR6,
            7 => RR7,
            8 => RPC,
            9 => RCOND,
            10 => RCOUNT,
            _ => RCOUNT,

        }


    }
}


pub enum OPCODE{
    OpBr, /* branch */
    OpAdd,    /* add  */
    OpLd,     /* load */
    OpSt,     /* store */
    OpJsr,    /* jump register */
    OpAnd,    /* bitwise and */
    OpLdr,    /* load register */
    OpStr,    /* store register */
    OpRti,    /* unused */
    OpNot,    /* bitwise not */
    OpLdi,    /* load indirect */
    OpSti,    /* store indirect */
    OpJmp,    /* jump */
    OpRes,    /* reserved (unused) */
    OpLea,    /* load effective address */
    OpTrap    /* execute trap */
}

impl From<OPCODE> for u16{
    fn from(value: OPCODE) -> Self {
       match value {
           OpBr=>0, /* branch */
           OpAdd=> 1,   /* add  */
           OpLd=> 2,    /* load */
           OpSt=>  3,   /* store */
           OpJsr=> 4,   /* jump register */
           OpAnd=>  5,  /* bitwise and */
           OpLdr=>  6,  /* load register */
           OpStr=>  7,  /* store register */
           OpRti=>  8,  /* unused */
           OpNot=>  9,  /* bitwise not */
           OpLdi=>  10,  /* load indirect */
           OpSti=>  11,  /* store indirect */
           OpJmp=>  12,  /* jump */
           OpRes=>  13,  /* reserved (unused) */
           OpLea=>   14, /* load effective address */
           OpTrap=>  15,  /* execute trap */
       }
    }
}

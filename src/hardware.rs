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


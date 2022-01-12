//10 Registers over which we have 8 general purpose registers and 2 with designate roles

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum Registers{
    Rr0 = 0,
    Rr1 = 1,
    Rr2 = 2,
    Rr3 = 3,
    Rr4 = 4,
    Rr5 = 5,
    Rr6 = 6,
    Rr7 = 7,
    Rr8 = 8,
    RPc , // Program Counter
    RCond, // Condition flag
    RCount,
}
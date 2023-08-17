

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Opcodes{
    OP_BR , /* branch */
    OP_ADD,    /* add  */
    OP_LD,     /* load */
    OP_ST,     /* store */
    OP_JSR,    /* jump register */
    OP_AND,    /* bitwise and */
    OP_LDR,    /* load register */
    OP_STR,    /* store register */
    OP_RTI,    /* unused */
    OP_NOT,    /* bitwise not */
    OP_LDI,    /* load indirect */
    OP_STI,    /* store indirect */
    OP_JMP,    /* jump */
    OP_RES,    /* reserved (unused) */
    OP_LEA,    /* load effective address */
    OP_TRAP    /* execute trap */
}

impl From<Opcodes> for i32 {
    fn from(value: Opcodes) -> Self {
        match value{
            Opcodes::OP_BR=> 0,
            Opcodes::OP_ADD=> 1,
            Opcodes::OP_LD=> 2,
            Opcodes::OP_ST=> 3,
            Opcodes::OP_JSR=> 4,
            Opcodes::OP_AND=> 5,
            Opcodes::OP_LDR=> 6,
            Opcodes::OP_STR=> 7,
            Opcodes::OP_RTI=> 8,
            Opcodes::OP_NOT=> 9,
            Opcodes::OP_LDI=> 10,
            Opcodes::OP_STI=> 11,
            Opcodes::OP_JMP=> 12,
            Opcodes::OP_RES=> 13,
            Opcodes::OP_LEA=> 14,
            Opcodes::OP_TRAP=> 15
        }
    }
}
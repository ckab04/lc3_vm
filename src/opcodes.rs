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
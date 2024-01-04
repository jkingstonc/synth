#[derive(Debug)]
pub enum InstructionType {
    NONE,
    // a block of code (i.e. this may be a lexical scope, a function block, etc)
    BLOCK,
    // integer literal (untyped as it could be u8, u32 etc)
    INT,
    // integer addition
    ADD,
    // integer subtraction
    SUB,
}

#[derive(Debug)]
pub struct InstructionData {}

// todo this should definitely be an enum, or maybe not :')
#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub data: InstructionData,
}

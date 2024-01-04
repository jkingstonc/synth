#[derive(Debug, Clone, Copy)]
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

// a ref refers to a location in the IR
#[derive(Debug, Clone, Copy)]
pub struct Ref {
    pub value: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionData {
    INT(i32),
    FLOAT(f32),
    DOUBLE_REF(Ref, Ref),
}

// todo this should definitely be an enum, or maybe not :')
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub data: InstructionData,
}

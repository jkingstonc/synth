#[derive(Debug)]
pub enum InstructionType {
    NONE,
    INT,
}

#[derive(Debug)]
pub struct InstructionData {}

// todo this should definitely be an enum, or maybe not :')
#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub data: InstructionData,
}

#[derive(Debug)]
pub enum InstructionType {
    NONE,
}

#[derive(Debug)]
pub struct InstructionData {}

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub data: InstructionData,
}

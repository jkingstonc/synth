/*


const x = 5
const y = 6
const z = x + y + 7

%x = int 5
%y = int 6
%0 = add %x %y
%z = add %0 7


*/

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
    // load instruction (todo this should depend on the type?)
    LOAD,
    // var instruction
    // this will allocate a variable some memory on the stack
    STACK_VAR,
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
    pub data: Option<InstructionData>,
}

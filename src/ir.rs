/*


const x = 5
const y = 6
const z = x + y + 7

%x = STACK_VAR 5
%y = STACK_VAR 6
%0 = LOAD %x
%1 = LOAD %y
%2 = add %0 %1
%z = add %2 7


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
#[derive(Debug, Clone)]
pub struct Ref {
    pub value: std::string::String,
}

#[derive(Debug, Clone)]
pub enum InstructionData {
    INT(i32),
    FLOAT(f32),
    REF(Ref),
    DOUBLE_REF(Ref, Ref),
}

// todo this should definitely be an enum, or maybe not :')
#[derive(Debug, Clone)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub data: Option<InstructionData>,
    // this is the value the struction assigns (i.e. %0 etc)
    pub assignment_name: Option<std::string::String>,
}

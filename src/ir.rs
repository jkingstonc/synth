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

// a ref refers to a location in the IR
#[derive(Debug, Clone)]
pub struct Ref {
    pub value: std::string::String,
}

// #[derive(Debug, Clone)]
// pub struct Bin {
//     pub left: InstructionData,
//     pub right: InstructionData,
// }

#[derive(Debug, Clone)]
pub enum InstructionData {
    REF(Ref),
    INT(i32),
    FLOAT(f32),
}

// todo this should definitely be an enum, or maybe not :')
#[derive(Debug, Clone)]
pub enum Instruction {
    // pub instruction_type: InstructionType,
    // pub data: Option<InstructionData>,
    // // this is the value the struction assigns (i.e. %0 etc)
    // pub assignment_name: Option<std::string::String>,
    NONE,
    BLOCK(std::string::String, Box<Vec<Instruction>>),
    // integer addition
    ADD(std::string::String, InstructionData, InstructionData),
    // integer subtraction
    SUB(std::string::String, InstructionData, InstructionData),
    // load instruction (todo this should depend on the type?)
    LOAD(std::string::String, Ref),
    // var instruction
    // this will allocate a variable some memory on the stack
    // for now it can not be initialised. this is fine as we know this will be on the stack
    // so can be assigned multiple times (not a SSA in registers)
    STACK_VAR(std::string::String, Option<InstructionData>),
    // conditional branch (as we are branching to other blocks this should be the last)
    // first arg is the condition, second is the body, third is the else
    COND_BR(InstructionData, Box<Instruction>),
}

impl Instruction {
    pub fn to_string_for_writing(&self) -> std::string::String {
        match self {
            Instruction::BLOCK(location, instruction_data) => {
                let mut s = "".to_string();
                for instruction in instruction_data.to_vec() {
                    s = s + &instruction.to_string_for_writing() + "\n";
                }
                format!("\n{:<10}\n{:<10}\n", location.to_string() + ":", s)
            }
            Instruction::LOAD(location, instruction_data) => {
                format!("{:<10} = {:<10} {:?}", location, "load", instruction_data)
            }
            Instruction::STACK_VAR(location, instruction_data) => format!(
                "{:<10} = {:<10} {:?}",
                location, "stack_var", instruction_data
            ),
            Instruction::ADD(location, left, right) => {
                format!("{:<10} = {:<10} {:?} + {:?}", location, "add", left, right)
            }
            Instruction::COND_BR(condition, body) => {
                format!(
                    "{:<10} {:?} then {}",
                    "if",
                    condition,
                    body.to_string_for_writing()
                )
            }
            _ => panic!(),
        }

        // "todo".to_string()
        // if let Some(assignment_name) = &self.assignment_name {
        //     if let Some(data) = &self.data {
        //         format!(
        //             "{:<10} = {:<10} {}",
        //             assignment_name,
        //             self.instruction_type.to_string(),
        //             data.to_owned().to_string_for_writing()
        //         )
        //     } else {
        //         format!(
        //             "{:<10} = {:<10}",
        //             assignment_name,
        //             self.instruction_type.to_string()
        //         )
        //     }
        // } else {
        //     if let Some(data) = &self.data {
        //         format!(
        //             "{:<10} {:<10} {}",
        //             "",
        //             self.instruction_type.to_string(),
        //             data.to_owned().to_string_for_writing()
        //         )
        //     } else {
        //         format!("{:<10} {:<10}", "", self.instruction_type.to_string())
        //     }
        // }
    }
}

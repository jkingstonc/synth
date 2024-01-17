use crate::types::Type;

// a ref refers to a location in memory (this is abstracted away, it could be a register, the stack etc. it's up to the code-generator to decide that)
#[derive(Debug, Clone)]
pub struct Ref {
    pub value: String,
}

// #[derive(Debug, Clone)]
// pub struct Bin {
//     pub left: IRValue,
//     pub right: IRValue,
// }

#[derive(Debug, Clone)]
pub enum IRValue {
    REF(Ref),
    INT(i32),
    FLOAT(f32),
    STRING(String),
    STRUCT(Vec<IRValue>),
    // todo this is a hack
    INTRINSIC(String),
}

// todo this should definitely be an enum, or maybe not :')
#[derive(Debug, Clone)]
pub enum Instruction {
    // pub instruction_type: InstructionType,
    // pub data: Option<IRValue>,
    // // this is the value the struction assigns (i.e. %0 etc)
    // pub assignment_name: Option<String>,
    NONE,
    PROGRAM(Box<Vec<Instruction>>),
    BLOCK(String, Box<Vec<Instruction>>),
    // integer addition
    ADD(String, IRValue, IRValue),
    // integer subtraction
    SUB(String, IRValue, IRValue),
    // load instruction (todo this should depend on the type?)
    LOAD(String, Ref),
    // store a value in a ref value
    STORE(Ref, IRValue),
    // var instruction
    // this will allocate a variable some memory on the stack
    // for now it can not be initialised. this is fine as we know this will be on the stack
    // so can be assigned multiple times (not a SSA in registers)
    STACK_VAR(String, Type, Option<IRValue>),
    // conditional branch (as we are branching to other blocks this should be the last)
    // first arg is the condition, second is the body, third is the else
    COND_BR(IRValue, Box<Instruction>, Option<Box<Instruction>>),
    // first arg is the function to call, the second is the first param (todo support more params)
    CALL(String, String, Vec<IRValue>),
    FUNC(String, Box<Instruction>),
    // todo need to decide if this is typed/untyped!
    // todo for now this is just the type but we may want the identifier?
    TYPE(String, Vec<Type>),
}

impl Instruction {
    pub fn to_string_for_writing(&self) -> String {
        match self {
            Instruction::BLOCK(location, instruction_data) => {
                let mut s = "".to_string();
                for instruction in instruction_data.to_vec() {
                    s = s + &instruction.to_string_for_writing() + "\n";
                }
                format!("\n{:<15}\n{:<10}\n", location.to_string() + ":", s)
            }
            Instruction::PROGRAM(instructions) => {
                let mut s = "".to_string();
                for instruction in instructions.to_vec() {
                    s = s + &instruction.to_string_for_writing() + "\n";
                }
                s
            }
            Instruction::LOAD(location, instruction_data) => {
                format!("{:<15} = {:<10} {:?}", location, "load", instruction_data)
            }
            Instruction::STORE(the_storee, value) => {
                format!("         {:<10} {:?} {:?}", "store", the_storee, value)
            }
            Instruction::STACK_VAR(location, typ, instruction_data) => format!(
                "{:<15} = {:<10} {:?} {:?}",
                location, "stack_var", typ, instruction_data
            ),
            Instruction::ADD(location, left, right) => {
                format!("{:<15} = {:<10} {:?} + {:?}", location, "add", left, right)
            }
            Instruction::CALL(location, callee, arg) => {
                format!(
                    "{:<15} = {:<10} {} args [{:?}]",
                    location, "call", callee, arg
                )
            }
            Instruction::FUNC(name, instructions) => {
                format!("def {:<15} = {:?}", name, instructions)
            }
            Instruction::TYPE(label, types) => {
                format!("{:<15} = type {:?}", label, types)
            }
            Instruction::COND_BR(condition, body, else_body) => {
                if let Some(else_body_unwrapped) = else_body {
                    format!(
                        "{:<15} {:?} then {} else {}",
                        "if",
                        condition,
                        body.to_string_for_writing(),
                        else_body_unwrapped.to_string_for_writing()
                    )
                } else {
                    format!(
                        "{:<15} {:?} then {}",
                        "if",
                        condition,
                        body.to_string_for_writing()
                    )
                }
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::IRValue;

    #[test]
    fn can_construct_int_data() {
        let int_data = IRValue::INT(123);
        assert_eq!(true, true);
    }
}

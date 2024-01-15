#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnPrimative {
    pub args: Vec<Type>,
    pub return_type: Option<Box<Type>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    U32,
    I32,
    F32,
    BOOL,
    STRING,
    SLICE,
    FN(FnPrimative),
    BLOCK,
    TYPE,
    STRUCT,
}

impl Type {
    pub fn size_in_bytes(&self) -> usize {
        match self {
            Type::U32 => 4,
            Type::I32 => 4,
            Type::F32 => 4,
            Type::STRUCT => todo!("size of struct"),
            _ => panic!("unknown type"),
        }
    }
}

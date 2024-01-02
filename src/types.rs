

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct FnPrimative {
    pub args: Vec<Type>,
    pub return_type: Option<Box<Type>>
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Primative {
    U32,
    I32,
    F32,
    BOOL,
    STRING,
    SLICE,
    FN(FnPrimative),
    BLOCK,
    TYPE,
    STRUCT
}


#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Type {
    pub primative: Primative
}


impl Type {
    pub fn size_in_bytes(&self) -> usize {
        match self.primative.to_owned() {
            Primative::U32 => 4,
            Primative::I32 => 4,
            Primative::F32 => 4,
            Primative::STRUCT => todo!("size of struct"),
            _ => panic!("unknown type")
        }
    } 
}



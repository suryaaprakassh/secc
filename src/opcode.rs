use std::fmt::{self, Formatter};

#[derive(Clone, Copy)]
pub enum OpCode {
    Constant(usize), //index of the constant
    Negate,
    Return,
}

pub type Value = f64;

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let std = match self {
            OpCode::Return => "return",
            OpCode::Negate => "-",
            OpCode::Constant(_) => "const",
        };
        f.write_str(std)
    }
}

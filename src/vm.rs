use crate::{
    common::Chunk,
    opcode::{self, OpCode},
};

pub struct Vm {
    chunks: Chunk,
    stack: Vec<opcode::Value>,
    ip: usize,
}

#[derive(Debug)]
pub enum InterpretRes {
    Ok,
    Failed,
}

type VmResult = Result<InterpretRes, Box<dyn std::error::Error>>;

impl Vm {
    pub fn new(chunks: Chunk) -> Self {
        Vm {
            chunks,
            stack: Vec::new(),
            ip: 0,
        }
    }

    pub fn interpret(&mut self) -> VmResult {
        while self.ip < self.chunks.code.len() {
            self.run(self.chunks.code[self.ip])?;
            self.ip+=1;
        }
        Ok(InterpretRes::Ok)
    }

    fn run(&mut self,code:opcode::OpCode) -> VmResult {
        use OpCode::*;
        match code {
            Return => return Ok(InterpretRes::Ok),
            Constant(idx) => {
                let val = self.chunks.get_value(idx)?;
                self.stack.push(val);
                println!("Constant {}", val);
            }
            Negate => {
                let val = self.stack.pop().unwrap();
                self.stack.push(-1.0 * val);
            }
        }
        Ok(InterpretRes::Failed)
    }
}

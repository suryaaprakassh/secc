use crate::opcode;
use std::io::{Error, ErrorKind};

pub struct Chunk {
    pub code: Vec<opcode::OpCode>,
    value_array: Vec<opcode::Value>,
    value_idx: usize,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            value_array: Vec::new(),
            value_idx: 0,
        }
    }
    pub fn write_code(&mut self, code: opcode::OpCode) {
        self.code.push(code);
    }

    pub fn write_value(&mut self, value: opcode::Value) -> usize {
        self.value_array.push(value);
        self.value_idx += 1;
        self.value_idx - 1
    }

    pub fn get_value(&self, idx: usize) -> Result<opcode::Value, std::io::Error> {
        match self.value_array.get(idx) {
            Some(val) => Ok(val.clone()),
            None => Err(Error::new(ErrorKind::Other, format!("Value not found!"))),
        }
    }

    pub fn dissassemble(&self) {
        let mut idx: usize = 0;
        for (_, op) in self.code.iter().enumerate() {
            let (format_op, idx_inc) = self.format_opcode(op);
            println!("{:04} {}", idx, format_op);
            idx += idx_inc;
        }
    }

    fn format_opcode(&self, op: &opcode::OpCode) -> (String, usize) {
        use opcode::OpCode::*;
        match op {
            Constant(idx) => {
                if let Some(val) = self.value_array.get(*idx) {
                    (format!("{} {} ({})", op, idx, val), 2)
                } else {
                    (format!("{} {} ({})", op, idx, "Invalid Index"), 1)
                }
            }
            _ => (format!("{}", op), 1),
        }
    }
}

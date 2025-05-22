use common::Chunk;
use opcode::OpCode;
use vm::Vm;

mod common;
mod opcode;
mod vm;

fn main() {
    let mut ck = Chunk::new();
    let idx = ck.write_value(2.0);
    ck.write_code(OpCode::Constant(idx));
    ck.write_code(OpCode::Negate);
    ck.write_code(OpCode::Return);
    ck.dissassemble();

    let mut vm = Vm::new(ck);
    match vm.interpret() {
        Ok(res) => {
            println!("{:?}", res)
        }
        Err(_) => {
            println!("Fked!")
        }
    }
}

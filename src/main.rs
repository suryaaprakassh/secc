use sexc::{common::read_file, lexer::Lexer, utils::KeyWordManager};

fn main() {
    let file_content = read_file().expect("Fked!");
    let keyword_manager = KeyWordManager::new();
    let mut lexer = Lexer::new(file_content, keyword_manager);
    let tokens = lexer.parse();
    dbg!(tokens);
    // let mut ck = Chunk::new();
    // let idx = ck.write_value(2.0);
    // ck.write_code(OpCode::Constant(idx));
    // ck.write_code(OpCode::Negate);
    // ck.write_code(OpCode::Return);
    // ck.dissassemble();
    //
    // let mut vm = Vm::new(ck);
    // match vm.interpret() {
    //     Ok(res) => {
    //         println!("{:?}", res)
    //     }
    //     Err(_) => {
    //         println!("Fked!")
    //     }
    // }
}

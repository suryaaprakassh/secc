#![allow(unused)]

use std::{env, fs, io::Read};

use common::Chunk;
use lexer::Lexer;
use opcode::OpCode;
use utils::KeyWordManager;
use vm::Vm;

mod common;
mod errors;
mod lexer;
mod opcode;
mod utils;
mod vm;

struct File {
    pub content: String,
    pub path: String,
}

impl File {
    pub fn get_ch(&self, idx: usize) -> Option<&u8> {
        if idx < self.content.len() {
            return self.content.as_bytes().get(idx);
        }
        None
    }
}

impl File {
    pub fn new(content: String, path: String) -> Self {
        Self { content, path }
    }

    pub fn slice(&self, start: usize, end: usize) -> String {
        self.content[start..end].to_string()
    }
}

fn read_file() -> Result<File, std::io::Error> {
    let file_name: Vec<String> = env::args().collect();
    println!("Tried to Read {}", file_name[1]);
    let mut file = fs::File::open(&file_name[1])?;
    let mut content = String::new();
    let n_bytes = file.read_to_string(&mut content)?;
    println!("Read Bytes {n_bytes}");
    Ok(File::new(content, file_name[1].to_owned()))
}

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

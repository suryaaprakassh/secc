use std::{fs::File, io::Read};

use sexc::lexer::Lexer;

fn main() {
    let mut file = File::open("./test.sc").unwrap();
    
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    
    let mut tokenizer = Lexer::new(content);
    
    let tokens = tokenizer.lex();

    println!("{:?}",tokens);
}

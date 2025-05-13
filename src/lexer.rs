
#![allow(dead_code)]

use crate::token::Token;


pub struct Lexer {
    source: String,
    position: usize,
    tokens : Vec<Token>
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Lexer { 
            source: src,
            position:0,
            tokens:Vec::new()
        }
    }

    fn consume(&mut self) -> Result<u8,()> {
        if self.position < self.source.len() {
            self.position+=1;
            return Ok(self.source.as_bytes()[self.position-1]);
        }
        Err(())
    }

    fn current(&self) -> Option<u8> {
        if self.position < self.source.len() {
            return Some(self.source.as_bytes()[self.position]);
        }
        None
    }

    fn peek(&self) -> Option<u8> {
        if self.position+1 < self.source.len() {
            return Some(self.source.as_bytes()[self.position+1] );
        }
        None
    }

    fn consume_numeric(&mut self) -> Result<(),()> {
        
        Ok(())
    } 

    fn consume_string(&mut self) -> Result<(),()> {
        let mut lexeme = String::new();
        while let Some(ch) = self.current(){
            if !ch.is_ascii_alphabetic(){
                break;
            }
            let ch = self.consume()?;
            lexeme.push(ch as char);

        }
        match lexeme.as_str() {
            "const" => {
                self.tokens.push(Token::Const)
            },
            "let" => {
                self.tokens.push(Token::Let)
            },

            "int" => {
                self.tokens.push(Token::IntType)
            }

            "float" => {
                self.tokens.push(Token::FloatType)
            }

            "string" => {
                self.tokens.push(Token::StringType);
            }

            "fn" => {
                self.tokens.push(Token::Function);
            }

            "true" => {
                self.tokens.push(Token::BoolLiteral(true));
            }

            "false" => {
                self.tokens.push(Token::BoolLiteral(false));
            }

            "return" => {
                self.tokens.push(Token::Return);
            }

            _ => {
                self.tokens.push(Token::Identifier(lexeme))
            }
        }
        return Ok(())
    }

    fn consume_symbols(&mut self)->Result<(),()>{
        let ch = self.consume()?;
        match char::from(ch) {
            ')' => {
                self.tokens.push(Token::RightParan)
            }
            '(' => {
                self.tokens.push(Token::LeftParan)
            }
            '{' => {
                self.tokens.push(Token::LeftCurl)
            }
            '}' => {
                self.tokens.push(Token::RightParan)
            }
            ',' => {
                self.tokens.push(Token::Comma)
            }
            ';' => {
                self.tokens.push(Token::SemiColon)
            }
            '=' => {
                self.tokens.push(Token::Equal)
            }
            '+' => {
                self.tokens.push(Token::Plus)
            }
            '-' => {
                self.tokens.push(Token::Minus)
            }
            '*' => {
                self.tokens.push(Token::Mult)
            }
            '/' => {
                self.tokens.push(Token::Divide)
            }
            ' '| '\n'=> {
                return Ok(());
            }
            _ => {
                println!("Symbol:{}",ch);
                return Err(())
            }
        }
        Ok(())
    }

    pub fn lex(&mut self) -> Result<Vec<Token>,()>{
        while let Some(ch) = self.current() {
            if ch.is_ascii_alphabetic() {
                if let Err(_) = self.consume_string(){
                    println!("Cannot consume string!");
                    return Err(());
                }
            }
            else if ch.is_ascii_digit() {
                if let Err(_) = self.consume_numeric() {
                    println!("Cannot consume int!");
                    return Err(());
                }
            }
            else {
                if let Err(_) = self.consume_symbols() {
                    println!("Unknown Symbol!");
                    return Err(());
                }
            }

        }
        Ok(self.tokens.clone())
    }
}

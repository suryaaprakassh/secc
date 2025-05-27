use crate::File;
use crate::errors::LexerError;
use crate::utils::KeyWordManager;

macro_rules! match_lex {
    ($producer:expr,$peek: expr ,$lft: expr ,$rht: expr) => {{
        if matches!($producer, Some(x) if *x as char  == $peek){
            $lft
        } else {
            $rht
        }
    }};
}

#[derive(Debug, Clone)]
pub enum Token {
    //operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    DoubleEqual,
    Not,
    BangEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    LeftParan,
    RightParan,
    LeftCurly,
    RightCurly,
    LeftSquare,
    RightSquare,
    Comma,
    SemiColon,

    //keywords
    Let,
    Const,
    Func,
    If,
    For,
    While,
    Struct,

    //Literals
    Int(i32),
    Float(f64),
    String(String),
}
pub struct Lexer {
    file: File,
    line_number: usize,
    pos: usize,
    tokens: Vec<Token>,
    keyword_manager: KeyWordManager,
}

impl Lexer {
    pub fn new(file: File, manager: KeyWordManager) -> Self {
        Self {
            file,
            line_number: 0,
            pos: 0,
            tokens: Vec::new(),
            keyword_manager: manager,
        }
    }

    pub fn peek(&self) -> Option<&u8> {
        self.file.get_ch(self.pos)
    }

    pub fn peek_next(&self) -> Option<&u8> {
        self.file.get_ch(self.pos + 1)
    }

    pub fn advance(&mut self) -> Option<&u8> {
        if let Some(ch) = self.file.get_ch(self.pos) {
            self.pos += 1;
            return Some(ch);
        }
        None
    }

    fn alphabet(&mut self) {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() || *ch == b'_' {
                self.advance();
            }else{
                break;
            }
        } 
        let lexemme = self.file.slice(start, self.pos + 1);
        dbg!(lexemme);
        todo!("Not done yet alphabet!")
    }

    fn numeric(&mut self) {
        self.advance().unwrap();
        todo!()
    }

    fn string_literal(&mut self) -> Result<Token, LexerError> {
        let start = self.pos;
        loop {
            match self.advance() {
                Some(ch) => {
                    if *ch == b'"' {
                        break;
                    }
                }
                None => return Err(LexerError::new("Expected '\"' in line xx")),
            }
        }
        Ok(Token::String(self.file.slice(start, self.pos)))
    }

    fn symbol(&mut self) -> Result<(), LexerError> {
        match self.advance() {
            Some(ch) => {
                let token = match *ch as char {
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Multiply,
                    '/' => {
                        if matches!(self.peek(), Some(b'/') | Some(b'*')) {
                            todo!("Implement Comments!")
                        } else {
                            Token::Divide
                        }
                    }
                    '=' => {
                        match_lex!(self.peek(), '=', Token::DoubleEqual, Token::Equal)
                    }
                    '!' => {
                        match_lex!(self.peek(), '=', Token::BangEqual, Token::Not)
                    }
                    '<' => {
                        match_lex!(self.peek(), '=', Token::LessEqual, Token::LessThan)
                    }
                    '>' => {
                        match_lex!(self.peek(), '=', Token::GreaterEqual, Token::GreaterThan)
                    }
                    '(' => Token::LeftParan,
                    ')' => Token::RightParan,
                    '{' => Token::LeftCurly,
                    '}' => Token::RightCurly,
                    '[' => Token::LeftSquare,
                    ']' => Token::RightSquare,
                    ',' => Token::Comma,
                    ';' => Token::SemiColon,
                    '"' => self.string_literal()?,
                    ch => {
                        return Err(LexerError::new(format!("Unknown Symbol {}!", ch).as_str()));
                    }
                };

                self.tokens.push(token);
                Ok(())
            }
            None => Err(LexerError::new("Expected Symbol: Found Nothing!")),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        while let Some(ch) = self.peek() {
            if (ch.is_ascii_alphabetic() || *ch == b'_') {
                self.alphabet();
            } else if (ch.is_ascii_digit()) {
                self.numeric();
            } else {
                self.symbol();
            }
        }
        self.tokens.to_owned()
    }
}

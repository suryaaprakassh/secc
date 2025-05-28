mod token;
use crate::match_lex;
use std::collections::HashMap;

use crate::common::File;
use crate::errors::LexerError;
use crate::lexer::token::Token;
pub struct Lexer {
    file: File,
    line_number: usize,
    pos: usize,
    tokens: Vec<Token>,
    keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(file: File) -> Self {
        let mut keywords: HashMap<String, Token> = HashMap::new();
        keywords.insert("let".to_string(), Token::Let);
        keywords.insert("const".to_string(), Token::Const);
        keywords.insert("fn".to_string(), Token::Func);
        keywords.insert("for".to_string(), Token::For);
        keywords.insert("if".to_string(), Token::If);
        keywords.insert("while".to_string(), Token::While);
        keywords.insert("struct".to_string(), Token::Struct);

        //types
        keywords.insert("int".to_string(), Token::IntType);
        keywords.insert("double".to_string(), Token::FloatType);
        keywords.insert("char".to_string(), Token::CharType);
        keywords.insert("string".to_string(), Token::StringType);

        Self {
            file,
            line_number: 1,
            pos: 0,
            tokens: Vec::new(),
            keywords,
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
            } else {
                break;
            }
        }
        let lexemme = self.file.slice(start, self.pos);

        if let Some(val) = self.keywords.get(&lexemme) {
            self.tokens.push(val.clone());
        } else {
            self.tokens.push(Token::Identifer(lexemme))
        }
    }

    fn numeric(&mut self) -> Result<(), LexerError> {
        let mut is_float = false;
        let mut digit: i32 = 0;
        let mut mantisa: f64= 0.0;
        while let Some(ch) = self.peek() {
            if !ch.is_ascii_digit() && *ch != b'.' {
                break;
            }
            match self.advance() {
                Some(ch) => {
                    if *ch == b'.' {
                        if is_float {
                            return Err(LexerError::new("Unknown symbol '.' expected a digit!"));
                        }
                        println!("Here!");
                        is_float = true;
                    }
                    else if is_float {
                        mantisa = (ch - 48) as f64; //48 ascii value of zero ig!
                        mantisa /= 10.0;

                    } else {
                        digit = (digit * 10) + (ch - 48) as i32;
                    }
                }
                None => {
                    break;
                }
            }
        }

        if !is_float {
            self.tokens.push(Token::Int(digit));
            return Ok(());
        }

        let literal = digit as f64;
        self.tokens.push(Token::Float(literal + mantisa));
        return Ok(());
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
                None => {
                    return Err(LexerError::new(
                        format!("Expected '\"' in line {}", self.line_number).as_str(),
                    ));
                }
            }
        }
        Ok(Token::String(self.file.slice(start, self.pos-1)))
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
                        match_lex!(
                            self.peek(),
                            '=',
                            {
                                self.advance();
                                Token::DoubleEqual
                            },
                            { Token::Equal }
                        )
                    }
                    '!' => {
                        match_lex!(
                            self.peek(),
                            '=',
                            {
                                self.advance();
                                Token::BangEqual
                            },
                            { Token::Not }
                        )
                    }
                    '<' => {
                        match_lex!(
                            self.peek(),
                            '=',
                            {
                                self.advance();
                                Token::LessEqual
                            },
                            { Token::LessThan }
                        )
                    }
                    '>' => {
                        match_lex!(
                            self.peek(),
                            '=',
                            {
                                self.advance();
                                Token::GreaterEqual
                            },
                            { Token::GreaterThan }
                        )
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

    pub fn handle_whitespace(&mut self) {
        if let Some(ch) = self.advance() {
            match *ch as char {
                '\n' | '\r' => {
                    self.line_number += 1;
                }
                _ => {}
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() || *ch == b'_' {
                self.alphabet();
            } else if ch.is_ascii_digit() {
                self.numeric().unwrap();
            } else if ch.is_ascii_whitespace() {
                self.handle_whitespace();
            } else {
                self.symbol().unwrap();
            }
        }
        self.tokens.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test_symbols() {
        let file = File::get_mock_file("+-* = /==!=<<=>>=;,");
        let mut lexer = Lexer::new(file);
        let tokens = lexer.parse();
        let check = vec![
            Plus,
            Minus,
            Multiply,
            Equal,
            Divide,
            DoubleEqual,
            BangEqual,
            LessThan,
            LessEqual,
            GreaterThan,
            GreaterEqual,
            SemiColon,
            Comma,
        ];
        assert_eq!(tokens, check)
    }

    #[test]
    fn test_line_number() {
        let file = File::get_mock_file("+\n-\n*");
        let mut lexer = Lexer::new(file);
        let tokens = lexer.parse();
        let check = vec![Plus, Minus, Multiply];
        assert_eq!(tokens, check);
        assert_eq!(lexer.line_number, 3)
    }

    #[test]
    fn test_keywords() {
        let file = File::get_mock_file("let fn if while for struct test");
        let mut lexer = Lexer::new(file);
        let tokens = lexer.parse();
        let check = vec![
            Let,
            Func,
            If,
            While,
            For,
            Struct,
            Identifer("test".to_string()),
        ];
        assert_eq!(tokens, check);
        assert_eq!(lexer.line_number, 1)
    }

    #[test] 
    fn test_string_literal() {
        let file = File::get_mock_file(r#"let name = "test";"#);
        let mut lexer = Lexer::new(file);
        let tokens = lexer.parse();
        let check = vec![
            Let,
            Identifer("name".to_string()),
            Equal,
            String("test".to_string()),
            SemiColon
        ];
        assert_eq!(tokens, check);
        assert_eq!(lexer.line_number, 1)
    }

    #[test]
    fn test_expression() {
        let file = File::get_mock_file("(2 * 2 ) + 69 *72.8");
        let mut lexer = Lexer::new(file);
        let tokens = lexer.parse();
        let check = vec![
            LeftParan,
            Int(2),
            Multiply,
            Int(2),
            RightParan,
            Plus,
            Int(69),
            Multiply,
            Float(72.8),
        ];
        assert_eq!(tokens, check);
        assert_eq!(lexer.line_number, 1)
    }
}

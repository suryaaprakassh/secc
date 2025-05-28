#[macro_export]
macro_rules! match_lex {
    ($producer:expr,$peek: expr ,$lft: block,$rht: block) => {{
        if matches!($producer, Some(x) if *x as char  == $peek){
            $lft
        } else {
            $rht
        }
    }};
}

#[derive(Debug, Clone, PartialEq)]
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

    //types
    Struct,
    IntType,
    FloatType,
    StringType,
    CharType,
    BoolType,

    //Literals
    Int(i32),
    Float(f64),
    String(String),
    Identifer(String),
}

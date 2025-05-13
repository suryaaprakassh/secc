#[derive(Debug,Clone)]
pub enum Token {
    //symbols
    LeftParan,
    RightParan,
    LeftCurl,
    RightCurl,
    Comma,
    SemiColon,
    
    //operators
    Equal,
    Plus,
    Minus,
    Mult,
    Divide,
    FloorDivide,

    
    //keywords
    Function,
    IntType,
    FloatType,
    StringType,
    Return,
    Const,
    Let,


    //Literals 
    IntLiteral(i32),
    FloatLiteral(f64),
    BoolLiteral(bool),

    //identifier
    Identifier(String)
}

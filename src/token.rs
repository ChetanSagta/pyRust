#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub value: DataTypes 
}

#[derive(Debug, Clone)]
pub enum DataTypes{
    Char(char),
    Float(f32),
    Int(i32),
    Str(&'static str)
}

#[derive(Debug, Clone,PartialEq)]
pub enum TokenType{
    Comment,
    Compare,
    Divide,
    Equals,
    Literal,
    Minus,
    Newline,
    NotEquals,
    Number,
    Plus,
    Product,
    Space
}

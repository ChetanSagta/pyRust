#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub value: DataTypes 
}

#[derive(Debug, Clone)]
pub enum DataTypes{
    Int(i32),
    Str(&'static str),
    Float(f32),
    Char(char)
}

#[derive(Debug, Clone)]
pub enum TokenType{
    Plus,
    Minus,
    Product,
    Divide,
    Equals,
    Compare,
    NotEquals,
    Comment,
    Literal,
    Number,
    Space,
    Newline
}

#[derive(Debug)]
pub struct Token<'a>{
    pub token_type: TokenType,
    pub value: &'a str
}

#[derive(Debug)]
pub enum TokenType{
    Plus,
    Minus,
    Product,
    Divide,
    Equals,
    Compare,
    NotEquals,
    Comment,
    Literal
}

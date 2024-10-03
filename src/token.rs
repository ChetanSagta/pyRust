#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub value: String
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

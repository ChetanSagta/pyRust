use crate::token::{DataTypes, Token};


pub trait AstNode{
    fn parse(self: &Self);

    fn print(self: &Self);
}

#[derive(Debug)]
pub enum BinaryOp{
    PLUS,
    MINUS,
    DIVIDE,
    MULTIPLY
}

pub enum ComparisonOp{
    EQUAL,
    NOTEQUAL,
    LESS,
    GREATER,
    GREATEREQUAL,
    LESSEQUAL,
}

#[derive(Debug)]
pub struct BinaryExpr{
    pub binary_op: BinaryOp,
    pub previous: Token,
    pub next: Token
}

impl AstNode for BinaryExpr{
    fn parse(self: &Self){

    }

    fn print(self: &Self){
	println!("Binary Expression => Previous: {:?}, BinaryOP: {:?}, Next:{:?} ", self.previous, self.binary_op, self.next);

    }
}

struct Constant{
    value: i32,
    kind: Option<DataTypes>
}

impl AstNode for Constant{
    fn parse(self: &Self){
    }
    fn print(self: &Self){

    }
}

struct Ops{

}

// impl AstNode for Constant{
//     fn parse(){

//     }
// }

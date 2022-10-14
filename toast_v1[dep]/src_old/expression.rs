use crate::tokens::{Literal, Operator};

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock{
    pub expressions:Vec<Expression>
}
#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    pub name:String,
    pub value:Box<Expression>
}
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub value: Box<Expression>
}
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInFunction{
    Print,
    MathOperator{
        operator:Operator,

        /// top, then bottom
        arguments: Box<(Expression,Expression)>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltInConstant{
}
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltIn{
    Function(BuiltInFunction),
    Constant(BuiltInConstant)
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    CodeBlock(CodeBlock),
    Identifier(String),

    BuiltInIdentifier(BuiltIn),

    Definition(Definition),
    Literal(Literal),
    Call(Call),
}
impl Expression {
}

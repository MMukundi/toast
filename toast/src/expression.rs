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
    MathOperator(Operator)
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
    BuiltIn(BuiltIn),
    TopLevelExpression(TopLevelExpression),
}
impl Expression {
}
#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelExpression {
    Definition(Definition),
    Literal(Literal),
    Call(Call),
}
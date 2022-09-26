use crate::tokens::Literal;

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
pub enum BuiltIn{
    Print
}
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    CodeBlock(CodeBlock),
    Identifier(String),
    BuiltIn(BuiltIn),
    TopLevelExpression(TopLevelExpression),
}
#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelExpression {
    Definition(Definition),
    Literal(Literal),
    Call(Call),
}
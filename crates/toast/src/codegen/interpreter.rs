use std::collections::HashMap;
use std::ptr::write;
use crate::codegen::Backend;
use crate::expression::{BuiltIn, BuiltInFunction, CodeBlock, Expression, TopLevelExpression};

#[derive(Default)]
pub struct Interpreter {
    stack: Vec<Expression>,
    var_defs: HashMap<String,Expression>,
}

#[derive(Debug)]
pub enum CallError {
    UndefinedCallable,
    Uncallable
}

impl Interpreter {
    pub fn try_call(&mut self, expr:Expression)->Result<(),CallError>{
        match expr {
            Expression::BuiltIn(BuiltIn::Function(func))=>{
                match func {
                    BuiltInFunction::Print => {
                        let to_print = self.stack.pop().expect("Stack underflow");
                        println!("{:?}",to_print);
                    }
                }
            },
            Expression::CodeBlock(code)=> {
                for expr in code.expressions {
                    self.process(expr)
                }
            },
            Expression::Identifier(name) => {
                let value = self.var_defs.get(&name).cloned().ok_or(CallError::UndefinedCallable)?;
                self.try_call(value)?;
            }
            e=>{
                return Err(CallError::Uncallable);
            }
        };
        Ok(())
    }
}

impl Backend for Interpreter {
    fn process(&mut self, expression: Expression) {
        match expression {
            Expression::CodeBlock(_) => {panic!("Cannot interpret codeblocks yet")}
            Expression::Identifier(name) => {
                let value = self.var_defs.get(&name).cloned().expect(&format!("Undefined identifier: {name}"));
                self.stack.push(value);
            }
            Expression::BuiltIn(_) => {panic!("Cannot interpret all built-ins yet")}
            lit @ Expression::TopLevelExpression(TopLevelExpression::Literal(_)) => {
                self.stack.push(lit);
            }
            Expression::TopLevelExpression(TopLevelExpression::Definition(define)) => {
                let _old_value =self.var_defs.insert(define.name,*define.value);
            }
            Expression::TopLevelExpression(TopLevelExpression::Call(call)) => {
                self.try_call(*call.value).unwrap();
            }
        };
        println!("Variables: {:?}\nStack: {:?}",&self.var_defs,&self.stack)
    }
}
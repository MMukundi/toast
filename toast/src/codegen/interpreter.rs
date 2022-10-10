use std::collections::HashMap;
use std::ops::Add;
use std::ptr::write;
use crate::codegen::Backend;
use crate::expression::{BuiltIn, BuiltInFunction, CodeBlock, Expression, TopLevelExpression};
use crate::tokens::{NumericLiteral, Operator, Literal};

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

macro_rules! literals {
    ($a:pat, $b:pat) => {
        (
            Expression::TopLevelExpression(TopLevelExpression::Literal(Literal::Number($a))),
            Expression::TopLevelExpression(TopLevelExpression::Literal(Literal::Number($b)))
        )
    };
}
impl Interpreter {
    pub fn eval(&self, expr:Expression) -> Option<Expression> {
        let mut return_expr = Some(&expr);
        while let Some(Expression::Identifier(name)) = &return_expr {
            return_expr = self.var_defs.get(name);
        };
        return_expr.cloned()
    }
    #[inline]
    pub fn pop_eval(&mut self)->Option<Expression>{
        self.stack.pop().and_then(|e|self.eval(e))
    }
    pub fn try_call(&mut self, expr:Expression)->Result<(),CallError>{
        match expr {
            Expression::BuiltIn(BuiltIn::Function(func))=>{
                match func {
                    BuiltInFunction::Print => {
                        let to_print = self.pop_eval().expect("Stack underflow");
                        println!("{:?}",to_print);
                    }
                    BuiltInFunction::MathOperator(op) => {
                        let top = self.pop_eval().expect(&format!("Stack underflow; need 2 arguments for {:?}",op));
                        let bott = self.pop_eval().expect(&format!("Stack underflow; need 2 arguments for {:?}",op));
                        if let (
                            Expression::TopLevelExpression(TopLevelExpression::Literal(Literal::Number(n))),
                            Expression::TopLevelExpression(TopLevelExpression::Literal(Literal::Number(d))),
                        ) = (top,bott){
                            let val = match op {
                                Operator::Add => n+d,
                                Operator::Sub => n-d,
                                Operator::Mul => n*d,
                                Operator::Div => n/d,
                                Operator::Mod => n%d,
                            };   
                        } 
                        todo!();
                    }
                }
            },
            Expression::CodeBlock(code)=> {
                for expr in code.expressions {
                    self.process(expr)
                }
            },
            ident@Expression::Identifier(_) => {
                let value = self.eval(ident).ok_or(CallError::UndefinedCallable)?;
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
use std::cell::{RefCell, Ref};
use std::collections::HashMap;
use std::io::{Write, Read, BufRead};
use std::ops::Add;
use std::ptr::write;
use std::rc::Rc;
use crate::codegen::Backend;
use crate::expression::{BuiltIn, BuiltInFunction, CodeBlock, Expression};
use crate::parser::Parser;
use crate::tokenizer::Tokens;
use crate::tokens::{Operator, Literal};
use crate::numeric_literal::NumericLiteral;

pub trait Prompt:Write {
    fn write_prompt(&mut self);
    fn write_continue_prompt(&mut self);
}

#[derive(Default,Debug)]
pub struct Interpreter {
    stack: Vec<Expression>,
    var_defs: HashMap<String,Expression>,
}

pub struct Prompter<R,W>{
    read:R,
    write:W,
    is_new_prompt:Rc<RefCell<bool>>,
    buffer:String,
}
impl <R:BufRead,W:Prompt>  Prompter<R,W> {
    pub fn new(read:R,write:W)->Self{
        Self {
            read,
            write,
            is_new_prompt:Rc::new(RefCell::new(true)),
            buffer:String::default()
        }
    }
}

impl <R:BufRead,W:Prompt> Iterator for Prompter<R,W> {
    type Item=String;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.is_new_prompt.borrow() {
            self.write.write_prompt();
        }else {
            self.write.write_continue_prompt();
        }
        self.write.flush();

        *self.is_new_prompt.as_ref().borrow_mut() = false;

        self.read.read_line(&mut self.buffer).expect("Error reading line");
        Some(self.buffer.clone())
    }
}

impl Interpreter {
    pub fn run_in_prompter<R:BufRead,W:Prompt>(mut prompter:Prompter<R,W>) {
        let is_new_prompt = Rc::clone(&prompter.is_new_prompt);
        let input =prompter.map(|s|s.chars().chain(std::iter::once('\n')).collect::<Vec<_>>()).flatten();
        let tokens = Tokens::new(input);
        let mut expressions = Parser::new(tokens);

        let mut interpreter = Self::default();
        while let Some(expression) = expressions.next() {
            // println!("start"); std::io::stdout().flush();
            *is_new_prompt.as_ref().borrow_mut() = true;
            interpreter.process(expression);
            println!("[");
            for e in interpreter.stack.iter().rev() {
                println!(" {:?}",e);
            }
            println!("]");
            std::io::stdout().flush();
        }
    }
}

#[derive(Debug)]
pub enum CallError {
    UndefinedCallable,
    Uncallable,
    BadArguments,
}

impl Interpreter {
    pub fn resolve(&self, expr:Expression) -> Option<Expression> {
        let mut return_expr = Some(&expr);
        while let Some(Expression::Identifier(name)) = &return_expr {
            return_expr = self.var_defs.get(name);
        };
        return_expr.cloned()
    }
    #[inline]
    pub fn pop_resolve(&mut self)->Option<Expression>{
        let expression = self.stack.pop()?;
        self.resolve(expression)
    }
    pub fn try_call(&mut self, expr:Expression)->Result<(),CallError>{
        match expr {
            Expression::BuiltInIdentifier(BuiltIn::Function(func))=>{
                match func {
                    BuiltInFunction::Print => {
                        let to_print = self.pop_resolve().expect("Stack underflow");
                        dbg!(&to_print);
                        println!("<{:?}>",to_print); std::io::stdout().flush();
                    }
                    BuiltInFunction::MathOperator{operator,arguments} => {
                        // dbg!(&self);
                        if let (
                            Expression::Literal(Literal::Number(n)),
                            Expression::Literal(Literal::Number(d)),
                        ) = *arguments {
                            let val = match operator {
                                Operator::Add => n+d,
                                Operator::Sub => n-d,
                                Operator::Mul => n*d,
                                Operator::Div => n/d,
                                Operator::Mod => n%d,
                            };   
                            self.stack.push(Expression::Literal(Literal::Number(val)));
                        }else {
                            unimplemented!("Can only add numbers. Cannot add {:?}",arguments);
                            return Err(CallError::BadArguments);
                        }
                    }
                }
            },
            Expression::CodeBlock(code)=> {
                for expr in code.expressions {
                    self.process(expr)
                }
            },
            ident@Expression::Identifier(_) => {
                let value = self.resolve(ident).ok_or(CallError::UndefinedCallable)?;
                self.try_call(value)?;
            }
            _=>{
                return Err(CallError::Uncallable);
            }
        };
        Ok(())
    }
}

impl Backend for Interpreter {
    fn process(&mut self, expression: Expression) {
        // println!("Expr: {:?}",&expression);
        match expression {
            Expression::CodeBlock(_) => {panic!("Cannot interpret codeblocks yet")}
            Expression::Identifier(name) => {
                let value = self.var_defs.get(&name).cloned().expect(&format!("Undefined identifier: {name}"));
                self.stack.push(value);
            },
            Expression::BuiltInIdentifier(_) => {panic!("Cannot interpret all built-ins yet")}
            lit @ Expression::Literal(_) => {
                // println!("Literal: {:?}",&lit);
                self.stack.push(lit);
            }
            Expression::Definition(define) => {
                let _old_value =self.var_defs.insert(define.name,*define.value);
            }
            Expression::Call(call) => {
                self.try_call(*call.value).unwrap();
            }
        };
        // println!("Variables: {:?}\nStack: {:?}",&self.var_defs,&self.stack)
    }
}
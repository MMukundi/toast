pub mod interpreter;

use crate::expression::Expression;

pub trait Backend {
    fn process(&mut self, expression:Expression);
}
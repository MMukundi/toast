use crate::token::Token;

pub struct Parser<T> {
    tokens: T,
}
impl<T: Iterator<Item = Token>> Iterator for Parser<T> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

use std::iter::Peekable;
use std::ops::ControlFlow;

pub struct BorrowedFilter<'i,I:Iterator,F> {
    iter: &'i mut Peekable<I>,
    predicate: F,
}
impl <'i,I:Iterator,F:FnMut(&I::Item)->bool> BorrowedFilter<'i,I,F> {
    pub fn new(iter: &'i mut Peekable<I>,predicate:F)->Self{
        Self{
            iter,predicate
        }
    }
}

impl <'i,I:Iterator,F:FnMut(&I::Item)->bool> Iterator for BorrowedFilter<'i,I,F>{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let should_collect = self.iter.peek().map(&mut self.predicate).unwrap_or(false);
        if should_collect {
            self.iter.next()
        }else {
            None
        }
    }
}
pub struct BorrowedFilterMap<'i, I:Iterator,F> {
    iter: &'i mut Peekable<I>,
    map: F,
}
impl <'i,U, I:Iterator,F:FnMut(&I::Item)->Option<U>> BorrowedFilterMap<'i, I,F> {
    pub fn new(iter: &'i mut Peekable<I>,map:F)->Self{
        Self{
            iter,map
        }
    }
}
impl <U,I:Iterator,F:FnMut(&I::Item)->Option<U>> Iterator for BorrowedFilterMap<'_,I,F>{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.peek().and_then(&mut self.map).into_iter().inspect(|_|{
            self.iter.next();
        }).next()
    }
}
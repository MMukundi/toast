use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Or<L, R> {
    Left(L),
    Right(R),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrIter<L, R>(Or<L, R>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollapsedOrIter<L, R>(Or<L, R>);

impl<L, R> Or<L, R> {
    #[inline]
    pub fn left(self) -> Option<L> {
        match self {
            Or::Left(l) => Some(l),
            Or::Right(_) => None,
        }
    }
    #[inline]
    pub fn right(self) -> Option<R> {
        match self {
            Or::Left(_) => None,
            Or::Right(r) => Some(r),
        }
    }
    // #[inline]
    // pub fn is_left(&self) -> bool {
    //     matches!(self, Self::Left(_))
    // }
    // #[inline]
    // pub fn is_right(&self) -> bool {
    //     matches!(self, Self::Right(_))
    // }
    #[inline]
    pub fn flip(self) -> Or<R, L> {
        match self {
            Or::Left(new_r) => Or::Right(new_r),
            Or::Right(new_l) => Or::Left(new_l),
        }
    }

    #[inline]
    pub fn map_both<T, U, F: FnOnce(L) -> T, G: FnOnce(R) -> U>(self, f: F, g: G) -> Or<T, U> {
        match self {
            Or::Left(l) => Or::Left(f(l)),
            Or::Right(r) => Or::Right(g(r)),
        }
    }
    #[inline]
    pub fn unify<T, F: FnOnce(L) -> T, G: FnOnce(R) -> T>(self, f: F, g: G) -> T {
        match self {
            Or::Left(l) => f(l),
            Or::Right(r) => g(r),
        }
    }
}
impl<T> Or<T, T> {
    #[inline]
    pub fn collapse(self) -> T {
        match self {
            Or::Left(l) => l,
            Or::Right(r) => r,
        }
    }
}
pub trait CollapseRight<T> {
    fn collapse_right(self) -> T;
}

impl <L,R:CollapseRight<L>> CollapseRight<L> for Or<L,R> {
    #[inline]
    fn collapse_right(self) -> L {
        match self {
            Or::Left(l) => l,
            Or::Right(r) => r.collapse_right(),
        }
    }
}
impl <T> CollapseRight<T> for T {
    #[inline]
    fn collapse_right(self) -> T {
        self
    }
}
pub trait CollapseLeft<T> {
    fn collapse_left(self) -> T;
}

impl <L:CollapseLeft<R>,R> CollapseLeft<R> for Or<L,R> {
    #[inline]
    fn collapse_left(self) -> R {
        match self {
            Or::Left(l) => l.collapse_left(),
            Or::Right(r) => r,
        }
    }
}
impl <T> CollapseLeft<T> for T {
    #[inline]
    fn collapse_left(self) -> T {
        self
    }
}



impl<L: IntoIterator, R: IntoIterator<Item = L::Item>> Or<L, R> {
    pub fn iter_collapsed(self) -> CollapsedOrIter<L::IntoIter, R::IntoIter> {
        CollapsedOrIter(self.map_both(IntoIterator::into_iter, IntoIterator::into_iter))
    }
}
impl<L: Iterator, R: Iterator> Iterator for OrIter<L, R> {
    type Item = Or<L::Item, R::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Or::Left(l) => l.next().map(Or::Left),
            Or::Right(r) => r.next().map(Or::Right),
        }
    }
}
impl<L: IntoIterator, R: IntoIterator> IntoIterator for Or<L, R> {
    type Item = Or<L::Item, R::Item>;

    type IntoIter = OrIter<L::IntoIter, R::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        OrIter(match self {
            Or::Left(l) => Or::Left(l.into_iter()),
            Or::Right(r) => Or::Right(r.into_iter()),
        })
    }
}

impl<L: Iterator, R: Iterator<Item = L::Item>> Iterator for CollapsedOrIter<L, R> {
    type Item = L::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Or::Left(l) => l.next(),
            Or::Right(r) => r.next(),
        }
    }
}
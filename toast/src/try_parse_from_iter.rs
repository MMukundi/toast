use std::{fmt::Debug, iter::{Peekable}};

pub trait Peek {
    type Item;

    fn advance(&mut self) -> Option<Self::Item>;
    fn peek(&mut self) -> Option<&Self::Item>;

    fn peek_while<F: FnMut(&Self::Item) -> Option<U>, U>(self, f: F) -> PeekWhile<Self, F>
    where
        Self: Sized,
    {
        PeekWhile {
            peek: self,
            func: f,
        }
    }
}
impl<P: Peek> Peek for &mut P {
    type Item = P::Item;

    fn advance(&mut self) -> Option<Self::Item> {
        P::advance(self)
    }
    fn peek(&mut self) -> Option<&Self::Item> {
        P::peek(self)
    }
}
impl<I: Iterator> Peek for Peekable<I> {
    type Item = I::Item;

    fn advance(&mut self) -> Option<Self::Item> {
        self.next()
    }

    fn peek(&mut self) -> Option<&Self::Item> {
        Peekable::peek(self)
    }
}

#[derive(Debug,Clone)]
pub struct Counted<I> {
    inner:I,
    count:usize
}
impl <I> Counted<I> {
    pub fn new(inner:I)->Self{
        Self {
            inner,
            count:0
        }
    }
    pub fn index(&self)->usize{
        self.count
    }
    fn increment_count(&mut self){
        self.count+=1;
    }
}
impl <P:Peek> Peek for Counted<P>{
    type Item=P::Item;

    fn advance(&mut self) -> Option<Self::Item> {
        self.increment_count();
        P::advance(&mut self.inner)
    }

    fn peek(&mut self) -> Option<&Self::Item> {
        P::peek(&mut self.inner)
    }
}
impl <I:Iterator> Iterator for Counted<I>{
    type Item=I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.increment_count();
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

pub struct PeekWhile<P, F> {
    peek: P,
    func: F,
}
impl<P: Peek, F: FnMut(&P::Item) -> Option<U>, U> Iterator for PeekWhile<P, F>
where
    P::Item: Debug,
    U: Debug,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let peeked = self.peek.peek()?;
        let mapped_peek = (self.func)(peeked);
        if mapped_peek.is_some() {
            self.peek.advance();
        }
        mapped_peek
    }
}

pub trait TryParseFromPeek<T>: Sized {
    type Err;
    type ParseContext;
    fn try_parse_from_peek<P: Peek<Item = T>>(
        peek: &mut P,
        context: Self::ParseContext,
    ) -> Result<Self, Self::Err>;
}

macro_rules! try_parse_unsigned_from_iter {
    ($($unsigned_int:ty),*) => {
        $(
            impl TryParseFromPeek<char> for $unsigned_int {
                type Err = std::num::IntErrorKind;
                type ParseContext = u32;
                fn try_parse_from_peek<P:Peek<Item=char>>(chars: &mut P,radix:Self::ParseContext)-> Result<Self,Self::Err>{
                    let radix_as_self = radix as Self;
                    let mut digits = <&mut P as Peek>::peek_while(chars,|c| c.to_digit(radix).map(|d|d as Self));
                    if let Some(first) = digits.next(){
                        digits.try_fold(first as Self, |current_value,next_digit|{
                            (current_value as Self).checked_mul(radix_as_self).and_then(|n|n.checked_add(next_digit as Self)).ok_or(Self::Err::PosOverflow)
                        })
                    } else {
                        Err(Self::Err::InvalidDigit)
                    }
                }
            }
        )*
    };
}
try_parse_unsigned_from_iter!(u8, u16, u32, u64, u128, usize);
macro_rules! try_parse_signed_from_iter {
    ($($signed_int:ty),*) => {
        $(impl TryParseFromPeek<char> for $signed_int {
            type Err=std::num::IntErrorKind;

            type ParseContext=u32;

            fn try_parse_from_peek<P:Peek<Item=char>>(chars: &mut P,radix:Self::ParseContext)-> Result<Self,Self::Err> {
                match chars.peek() {
                    Some('-')=>{
                        chars.advance();
                        let radix_as_self = radix as Self;
                        let mut digits = <&mut P as Peek>::peek_while(chars,|c| c.to_digit(radix).map(|d|d as Self));
                        if let Some(first) = digits.next(){
                            digits.try_fold(-(first as Self), |current_value,next_digit|{
                                (current_value as Self).checked_mul(radix_as_self).and_then(|n|n.checked_sub(next_digit as Self)).ok_or(Self::Err::NegOverflow)
                            })
                        } else {
                            Err(Self::Err::InvalidDigit)
                        }
                    },
                    Some(c) if c.is_ascii_digit()=>{
                        let radix_as_self = radix as Self;
                        let mut digits = <&mut P as Peek>::peek_while(chars,|c| c.to_digit(radix).map(|d|d as Self));
                        if let Some(first) = digits.next(){
                            digits.try_fold(first as Self, |current_value,next_digit|{
                                (current_value as Self).checked_mul(radix_as_self).and_then(|n|n.checked_add(next_digit as Self)).ok_or(Self::Err::PosOverflow)
                            })
                        } else {
                            Err(Self::Err::InvalidDigit)
                        }
                    },
                    Some(_) =>{
                        Err(Self::Err::InvalidDigit)
                    },
                    _=>{
                        return Err(Self::Err::Empty)
                    }
                }

            }
        })*
    }
}
try_parse_signed_from_iter!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    #[macro_export]
    macro_rules! assert_try_parse_from_str_ok {
        ($T:ty, $source:expr => ($val:expr, $expected_remaining:expr); $radix:expr) => {
            {
                let mut chars = $source.chars().peekable();
                let res = <$T as super::TryParseFromPeek<char>>::try_parse_from_peek(&mut chars,$radix);
                assert_eq!(res, Ok($val));
                assert_eq!(chars.collect::<String>(),String::from($expected_remaining));
            }
        };
        ($T:ty,$source: expr => ($val:expr, $expected_remaining:expr)) => {
            assert_try_parse_from_str_ok!($T, $source => ($val, $expected_remaining); 10)
        };
    }
    #[macro_export]
    macro_rules! assert_try_parse_from_str_err {
        ($T:ty, $source:expr => $err:expr; $radix:expr) => {
            {
                let mut chars = $source.chars().peekable();
                let res = <$T as super::TryParseFromPeek<char>>::try_parse_from_peek(&mut chars,$radix);
                assert_eq!(res, Err($err))
            }
        };
        ($T:ty,$source: expr => $err:expr) => {
            assert_try_parse_from_str_err!($T, $source => $err; 10)
        };
    }
}
#[cfg(test)]
mod unsigned_tests {
    use crate::{assert_try_parse_from_str_err, assert_try_parse_from_str_ok};
    use std::num::IntErrorKind;
    #[test]
    fn assert_from_str() {
        assert_try_parse_from_str_ok!(u8, "255" => (255,""); 10);
    }
    #[test]
    fn assert_from_str_with_whitespace() {
        assert_try_parse_from_str_ok!(u8, "255   " => (255,"   "); 10);
    }
    #[test]
    fn assert_from_str_with_rest() {
        assert_try_parse_from_str_ok!(u8, "255abcdefg" => (255,"abcdefg"); 10);
    }
    #[test]
    fn assert_from_str_with_nums_in_binary() {
        assert_try_parse_from_str_ok!(u8, "1008" => (4,"8"); 2);
    }
    #[test]
    fn assert_from_str_err() {
        assert_try_parse_from_str_err!(u8, "256" => IntErrorKind::PosOverflow; 10);
    }

    #[test]
    fn assert_from_str_plus() {
        assert_try_parse_from_str_err!(u8, "+255" => IntErrorKind::InvalidDigit; 10);
    }
    #[test]
    fn assert_from_str_with_whitespace_plus() {
        assert_try_parse_from_str_err!(u8, "+255   " => IntErrorKind::InvalidDigit; 10);
    }
    #[test]
    fn assert_from_str_with_rest_plus() {
        assert_try_parse_from_str_err!(u8, "+255abcdefg" => IntErrorKind::InvalidDigit; 10);
    }
    #[test]
    fn assert_from_str_with_nums_in_binary_plus() {
        assert_try_parse_from_str_err!(u8, "+1008" => IntErrorKind::InvalidDigit; 2);
    }
    #[test]
    fn assert_from_str_err_plus() {
        assert_try_parse_from_str_err!(u8, "+256" => IntErrorKind::InvalidDigit; 10);
    }
}

#[cfg(test)]
mod signed_tests {
    use crate::{assert_try_parse_from_str_err, assert_try_parse_from_str_ok};
    use std::num::IntErrorKind;

    #[test]
    fn singed_assert_from_str() {
        assert_try_parse_from_str_ok!(i8, "127" => (127,""); 10);
    }
    #[test]
    fn singed_assert_from_str_with_whitespace() {
        assert_try_parse_from_str_ok!(i8, "127   " => (127,"   "); 10);
    }
    #[test]
    fn singed_assert_from_str_with_rest() {
        assert_try_parse_from_str_ok!(i8, "127abcdefg" => (127,"abcdefg"); 10);
    }
    #[test]
    fn singed_assert_from_str_with_nums_in_binary() {
        assert_try_parse_from_str_ok!(i8, "1008" => (4,"8"); 2);
    }
    #[test]
    fn singed_assert_from_str_err() {
        assert_try_parse_from_str_err!(i8, "128" => IntErrorKind::PosOverflow; 10);
    }

    #[test]
    fn singed_assert_from_str_plus() {
        assert_try_parse_from_str_err!(i8, "+127" => IntErrorKind::InvalidDigit; 10);
    }
    #[test]
    fn singed_assert_from_str_with_whitespace_plus() {
        assert_try_parse_from_str_err!(i8, "+127   " => IntErrorKind::InvalidDigit; 10);
    }
    #[test]
    fn singed_assert_from_str_with_rest_plus() {
        assert_try_parse_from_str_err!(i8, "+127abcdefg" => IntErrorKind::InvalidDigit; 10);
    }
    #[test]
    fn singed_assert_from_str_with_nums_in_binary_plus() {
        assert_try_parse_from_str_err!(i8, "+1008" => IntErrorKind::InvalidDigit; 2);
    }
    #[test]
    fn singed_assert_from_str_err_plus() {
        assert_try_parse_from_str_err!(i8, "+128" => IntErrorKind::InvalidDigit; 10);
    }

    #[test]
    fn singed_assert_from_str_minus() {
        assert_try_parse_from_str_ok!(i8, "-128" => (-128,""); 10);
    }
    #[test]
    fn singed_assert_from_str_with_whitespace_minus() {
        assert_try_parse_from_str_ok!(i8, "-128   " => (-128,"   "); 10);
    }
    #[test]
    fn singed_assert_from_str_with_rest_minus() {
        assert_try_parse_from_str_ok!(i8, "-128abcdefg" => (-128,"abcdefg"); 10);
    }
    #[test]
    fn singed_assert_from_str_with_nums_in_binary_minus() {
        assert_try_parse_from_str_ok!(i8, "-1008" => (-4,"8"); 2);
    }
    #[test]
    fn singed_assert_from_str_err_minus() {
        assert_try_parse_from_str_err!(i8, "-129" => IntErrorKind::NegOverflow; 10);
    }
}

use std::{iter::Rev, str::Chars, fmt::Debug};

pub trait Poppable:Debug {
    fn pop_first(&mut self) -> Option<char>;
}

impl Poppable for &str {
    fn pop_first(&mut self) -> Option<char> {
        let mut chars = self.chars();
        let first_char = chars.next();
        *self = chars.as_str();
        first_char
    }
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub struct PoppableString(String);
impl PoppableString {
    pub fn new(string: String) -> Self {
        PoppableString(string.chars().rev().collect())
    }
    pub fn into_string(self) -> String {
        self.0.chars().rev().collect()
    }
    pub fn chars(&self) -> Rev<Chars> {
        self.0.chars().rev()
    }
}
impl Poppable for PoppableString {
    fn pop_first(&mut self) -> Option<char> {
        self.0.pop()
    }
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub struct StringyChars<S> {
    stringy: S,
}
impl<S> StringyChars<S> {
    pub fn new(stringy: S) -> Self {
        Self { stringy }
    }
}
impl<S: Poppable> Iterator for StringyChars<S> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.stringy.pop_first()
    }
}

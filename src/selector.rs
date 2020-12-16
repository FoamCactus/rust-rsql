use crate::lex_item::LexItem;
use crate::Parseable;
use crate::ParseError;
use std::slice::Iter;
use std::iter::Peekable;

#[derive(PartialEq, Eq, Debug,Clone)]
pub struct Selector {
    value: String,
}

impl Parseable for Selector {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError> {
        let mut v = Vec::new();
        while input.peek().map_or(false, |l| l.is_char()) {
            v.push(input.next().unwrap().to_char());
        }
        if v.len() == 0 {
            Err(ParseError::EmptyIterator)
        } else {
            Ok(Self { value: v.into_iter().collect() })
        }
    }
}

impl Selector {
    pub fn new(val: String) -> Self {
        Self{value: val}
    }
}

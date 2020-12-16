use std::iter::Peekable;
use std::slice::Iter;
mod tests;
pub mod comparison;
pub mod value;
pub mod lex_item;
use lex_item::LexItem;
pub mod comparison_operator;
pub mod expression;
pub mod group;
pub mod logical_operator;
pub mod selector;


pub trait Parseable {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError>
    where
        Self: std::marker::Sized;
}

#[derive(Debug)]
pub enum ParseError {
    Error(&'static str),
    EmptyIterator,
    Unclosedquote,
    TokenNotAllowed(LexItem),
}




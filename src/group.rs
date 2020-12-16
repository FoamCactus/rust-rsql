use crate::{ParseError,Parseable};
use std::iter::Peekable;
use std::slice::Iter;
use crate::lex_item::LexItem;
use crate::expression::Expression;

#[derive(Debug, Clone)]
pub struct Group {
    val: Expression,
}
impl Parseable for Group {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError> {
        use LexItem::LeftParenthesis;
        match input.next() {
            Some(LeftParenthesis) => {
                let g = Group {
                    val: Expression::parse(input)?,
                };
                Ok(g)
            }
            Some(t) => Err(ParseError::TokenNotAllowed(*t)),
            None => Err(ParseError::EmptyIterator),
        }
    }
}

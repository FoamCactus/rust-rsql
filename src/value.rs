use crate::lex_item::LexItem;
use crate::Parseable;
use crate::ParseError;
use std::slice::Iter;
use std::iter::Peekable;



#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    SingleQuoteString(String),
    DoubleQuotedString(String),
    Unreserved(String),
}

impl Parseable for Value {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError> {
        use LexItem::*;
        if let Some(a) = input.next() {
            match a {
                SingleQuote => {
                    let mut s = Vec::new();
                    while input.peek().map_or(false, |l| l.is_char()) {
                        s.push(input.next().unwrap().to_char());
                    }
                    if let Some(SingleQuote) = input.next() {
                        Ok(Self::SingleQuoteString(s.into_iter().collect()))
                    } else {
                        Err(ParseError::Unclosedquote)
                    }
                }
                DoubleQuote => {
                    let mut s = Vec::new();
                    while input.peek().map_or(false, |l| l.is_char()) {
                        s.push(input.next().unwrap().to_char());
                    }
                    if let Some(DoubleQuote) = input.next() {
                        Ok(Self::DoubleQuotedString(s.into_iter().collect()))
                    } else {
                        Err(ParseError::Unclosedquote)
                    }
                }
                Char(c) => {
                    let mut s = vec![*c];
                    while input.peek().map_or(false, |l| l.is_char()) {
                        s.push(input.next().unwrap().to_char());
                    }
                    Ok(Self::Unreserved(s.into_iter().collect()))
                }
                t => Err(ParseError::TokenNotAllowed(*t)),
            }
        } else {
            Err(ParseError::EmptyIterator)
        }
    }
}

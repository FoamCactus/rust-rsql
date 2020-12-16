use crate::lex_item::LexItem;
use crate::{Parseable,ParseError};
use std::iter::Peekable;
use std::slice::Iter;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum LogicalOperator {
    And,
    Or,
    End,
}

impl Parseable for LogicalOperator {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError> {
        use LexItem::*;
        match input.next() {
            Some(Comma) => Ok(Self::Or),
            Some(SemiColon) => Ok(Self::And),
            Some(Char('o')) => match input.next() {
                Some(Char('r')) => Ok(Self::Or),
                Some(t) => Err(ParseError::TokenNotAllowed(*t)),
                None => Err(ParseError::EmptyIterator),
            },
            Some(Char('a')) => {
                let nd: String = input.take(2).map(|l| l.to_char()).collect();
                if "nd" == nd {
                    Ok(Self::And)
                } else {
                    Err(ParseError::TokenNotAllowed(
                        LexItem::parse_char(nd.chars().next().unwrap()).unwrap(),
                    ))
                }
            }
            Some(RightParenthesis) => Ok(Self::End),
            Some(t) => Err(ParseError::TokenNotAllowed(*t)),
            None => Ok(Self::End),
        }
    }
}


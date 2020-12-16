use either::Either;
use crate::comparison::Comparison;
use crate::logical_operator::LogicalOperator;
use std::iter::Peekable;
use std::slice::Iter;
use crate::lex_item::{LexItem,lex};
use crate::group::Group;
use crate::{Parseable,ParseError};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Expression {
    list: Vec<Either<(Comparison, LogicalOperator), Group>>,
}

impl Parseable for Expression {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError> {
        use LexItem::*;
        let mut list = Vec::new();
        let mut has_error: Option<LexItem> = None;
        while let Some(next) = input.peek() {
            match next {
                LeftParenthesis => {
                    let group = Group::parse(input)?;
                    list.push(Either::Right(group));
                }
                Char(_) => {
                    list.push(Either::Left((
                        Comparison::parse(input)?,
                        LogicalOperator::parse(input)?,
                    )));
                }
                t => {
                    has_error = Some(**t);
                }
            }
        }
        match has_error {
            Some(t) => Err(ParseError::TokenNotAllowed(t)),
            None => Ok(Self { list: list }),
        }
    }
}

impl Expression {

    fn groups(&self) -> Vec<Group> {
        use Either::*;
        self.list
            .iter()
            .filter_map(|r| {
                match r {
                    Right(a) => Some(a.clone()),
                    _ => None
                }
            })
            .collect()
    }

    fn comparisons(&self) ->  Vec<(Comparison,LogicalOperator)> {
        use Either::*;
        self.list
            .iter()
            .filter_map(|r| {
                match r {
                    Left(a) => Some(a.clone()),
                    _ => None
                }
            })
            .collect()
    }
}

impl TryFrom<String> for Expression {
    type Error = ParseError;
    fn try_from(input: String) -> Result<Self,Self::Error> {
        match lex(input) {
            Ok(s) => {
                Self::parse(&mut s.iter().peekable())
            }
            Err(_) => {
                Err(ParseError::Error("outer most error"))
            }
        }
    }
}

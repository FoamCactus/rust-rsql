use crate::{Parseable,ParseError};
use crate::lex_item::LexItem;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ComparisonOperators {
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
    In,
    Out,
}

impl Parseable for ComparisonOperators {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError> {
        use LexItem::*;
        if let Some(a) = input.next() {
            println!("current: {:?}, next: {:?}", a, input.peek());
            match a {
                EqualSign => {
                    let next = input.peek();
                    if let Some(Char('i')) = next {
                        let check: String = input.take(3).map(|l| l.to_char()).collect();
                        if check == "in=".to_string() {
                            Ok(Self::In)
                        } else {
                            Err(ParseError::Error("in error"))
                        }
                    } else if let Some(Char('l')) = next {
                        let check: String = input.take(3).map(|l| l.to_char()).collect();
                        if check == "lt=" {
                            Ok(Self::LessThan)
                        } else if check == "le=" {
                            Ok(Self::LessThanOrEqual)
                        } else {
                            Err(ParseError::Error("less than error"))
                        }
                    } else if let Some(Char('g')) = next {
                        let check: String = input.take(3).map(|l| l.to_char()).collect();
                        if check == "gt=" {
                            Ok(Self::GreaterThan)
                        } else if check == "ge=" {
                            Ok(Self::GreaterThanOrEqual)
                        } else {
                            Err(ParseError::Error("greater than error"))
                        }
                    } else if let Some(Char('o')) = next {
                        let check: String = input.take(4).map(|l| l.to_char()).collect();
                        if check == "out=".to_string() {
                            Ok(Self::Out)
                        } else {
                            Err(ParseError::Error("out error"))
                        }
                    } else if let Some(EqualSign) = next {
                        input.next();
                        println!("here");
                        Ok(Self::Equal)
                    } else {
                        match next {
                            Some(e) => Err(ParseError::TokenNotAllowed(**e)),
                            None => Err(ParseError::EmptyIterator),
                        }
                    }
                }
                LessThanSign => {
                    let next = input.peek();
                    if let Some(EqualSign) = next {
                        input.next();
                        Ok(Self::LessThanOrEqual)
                    } else {
                        Ok(Self::LessThan)
                    }
                }
                GreaterThanSign => {
                    let next = input.peek();
                    if let Some(EqualSign) = next {
                        input.next();
                        Ok(Self::GreaterThanOrEqual)
                    } else {
                        Ok(Self::GreaterThan)
                    }
                }
                ExclamationPoint => match input.next() {
                    Some(EqualSign) => Ok(Self::NotEqual),
                    Some(t) => Err(ParseError::TokenNotAllowed(*t)),
                    None => Err(ParseError::EmptyIterator),
                },
                t => Err(ParseError::TokenNotAllowed(*t)),
            }
        } else {
            Err(ParseError::EmptyIterator)
        }
    }
}


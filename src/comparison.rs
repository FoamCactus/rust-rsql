use crate::value::Value;
use crate::selector::Selector;
use crate::comparison_operator::ComparisonOperators;
use crate::lex_item::LexItem;
use crate::{Parseable,ParseError};
use std::iter::Peekable;
use std::slice::Iter;
#[derive(Debug, Clone)]
pub struct Comparison {
    first: Selector,
    comp: ComparisonOperators,
    second: Value,
}

impl Parseable for Comparison {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError> {
        let r = Self {
            first: Selector::parse(input)?,
            comp: ComparisonOperators::parse(input)?,
            second: Value::parse(input)?,
        };
        Ok(r)
    }
}




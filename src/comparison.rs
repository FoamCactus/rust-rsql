use crate::value::Value;
use crate::selector::Selector;
use crate::comparison_operator::ComparisonOperators;
use crate::lex_item::LexItem;
use crate::{Parseable,ParseError};
use std::iter::Peekable;
use std::slice::Iter;
#[derive(Debug, Clone)]
pub struct Comparison {
    selector: Selector,
    comp: ComparisonOperators,
    val: Value,
}

impl Parseable for Comparison {
    fn parse(input: &mut Peekable<Iter<LexItem>>) -> Result<Self, ParseError> {
        let r = Self {
            selector: Selector::parse(input)?,
            comp: ComparisonOperators::parse(input)?,
            val: Value::parse(input)?,
        };
        Ok(r)
    }
}

impl Comparison {
    pub fn get_comparison(&self) -> ComparisonOperators {
        self.comp
    }

    pub fn get_selector(&self) -> Selector {
        self.selector.clone()
    }

    pub fn get_value(&self) -> Value {
        self.val.clone()
    }

}




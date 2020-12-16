#[cfg(test)]
mod tests {
    use crate::lex_item::{LexItem,LexError,lex};
    use crate::{ParseError,Parseable};
    #[test]
    fn lex_all() {
        use LexItem::*;
        let a = "( =<>c;,)".to_string();
        let comp = vec![
            LeftParenthesis,
            Space,
            EqualSign,
            LessThanSign,
            GreaterThanSign,
            Char('c'),
            SemiColon,
            Comma,
            RightParenthesis,
        ];
        let out = lex(a);
        println!("{:?}", comp);
        println!("{:?}", out);
        if let Ok(out) = out {
            assert!(comp == out);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn selector_parse() -> Result<(), LexError> {
        use crate::selector::Selector;
        let hello = String::from("hello");
        let comp = Selector::new(hello.clone());
        if let Ok(s) = Selector::parse(&mut lex(hello)?.iter().peekable()) {
            assert!(s == comp);
        } else {
            assert!(false);
        }
        Ok(())
    }

    #[test]
    fn comp_parse() -> Result<(), LexError> {
        use crate::comparison_operator::ComparisonOperators;
        use crate::comparison_operator::ComparisonOperators::*;
        let all_strings = vec![
            "==", "<=", ">=", "=in=", "=out=", "=lt=", "=le=", "=gt=", "=ge=", "!=",
        ];
        let all = vec![
            Equal,
            LessThanOrEqual,
            GreaterThanOrEqual,
            In,
            Out,
            LessThan,
            LessThanOrEqual,
            GreaterThan,
            GreaterThanOrEqual,
            NotEqual,
        ];
        let iter = all_strings.iter().map(|s| String::from(*s)).zip(all.iter());
        for pair in iter {
            println!("{:?},{:?}", pair.0, pair.1);
            if let Ok(v) = ComparisonOperators::parse(&mut lex(pair.0)?.iter().peekable()) {
                assert!(v == *pair.1);
            } else {
                false;
            }
        }
        Ok(())
    }

    #[test]
    fn value_parse() -> Result<(), LexError> {
        use crate::value::Value; 
        use crate::value::Value::*;
        let all_strings = vec!["\"hello\"", "\'hello\'", "hello"];
        let val = String::from("hello");
        let all = vec![
            DoubleQuotedString(val.clone()),
            SingleQuoteString(val.clone()),
            Unreserved(val.clone()),
        ];
        let iter = all_strings.iter().map(|s| String::from(*s)).zip(all.iter());
        for pair in iter {
            println!("{:?},{:?}", pair.0, pair.1);
            if let Ok(v) = Value::parse(&mut lex(pair.0)?.iter().peekable()) {
                assert!(v == *pair.1);
            } else {
                false;
            }
        }
        Ok(())
    }

    #[test]
    fn logic_op_parse() -> Result<(), LexError> {
        use crate::logical_operator::LogicalOperator;
        use crate::logical_operator::LogicalOperator::*;
        let all_strings = vec![";", "and", ",", "or"];
        let all = vec![And, And, Or, Or];
        let iter = all_strings.iter().map(|s| String::from(*s)).zip(all.iter());
        for pair in iter {
            println!("{:?},{:?}", pair.0, pair.1);
            if let Ok(v) = LogicalOperator::parse(&mut lex(pair.0)?.iter().peekable()) {
                assert!(v == *pair.1);
            } else {
                false;
            }
        }
        Ok(())
    }

    #[test]
    fn basic_full() -> Result<(), ParseError> {
        use crate::expression::Expression;
        use std::convert::TryFrom;
        let query = String::from("id==45");
        match Expression::try_from(query) {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }

    #[test]
    fn group_basic() -> Result<(), ParseError> {
        use crate::expression::Expression;
        use std::convert::TryFrom;
        let query = String::from("id==45;(name==\"23\")");
        match Expression::try_from(query) {
            Ok(_) => assert!(true),
            Err(e) => {
                println!("error: {:?}", e);
                assert!(false);
            }
        }
        Ok(())
    }
}

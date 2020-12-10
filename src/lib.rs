#[cfg(test)]
mod tests {
    use crate::LexItem::*;
    use crate::*;
    #[test]
    fn lex_all() {
        let a = "( =<>c;,)".to_string();
        let comp = vec![LeftParenthesis,Space,EqualSign,LessThanSign,GreaterThanSign,Char('c'),SemiColon,Comma,RightParenthesis];
        let out = lex(a);
        println!("{:?}",comp);
        println!("{:?}",out);
        assert!(comp == out);
    }

}

#[derive(PartialEq,Eq,Debug)]
enum LexItem {
    LeftParenthesis,
    RightParenthesis,
    Space,
    EqualSign,
    LessThanSign,
    GreaterThanSign,
    Char(char),
    SemiColon,
    Comma
}
impl LexItem {
    pub fn parse_char(c: char) -> Result<Self,LexError> {
        use LexItem::*;
        match c {
            '(' => Ok(LeftParenthesis),
            ')' => Ok(RightParenthesis),
            ' ' => Ok(Space),
            '=' => Ok(EqualSign),
            '<' => Ok(LessThanSign),
            '>' => Ok(GreaterThanSign),
            'A'..='z' => Ok(Char(c)),
            ';' => Ok(SemiColon),
            ',' => Ok(Comma),
            _ => Err(LexError::InvalidCharacter(c))
        }
    }
}


#[derive(Debug)]
enum LexError {
    InvalidCharacter(char)
}


fn lex(input: String ) -> Vec<LexItem> {
    let tokens = input.chars().filter_map(|c| LexItem::parse_char(c).ok()).collect();
    tokens
}

pub struct Expression {

}

enum Comparison {
    LessThan,
    GreaterThan,
    Equal,
    LessThanOrEqual,
    GreaterThanOrEqual,
    In,
    Out
} 


impl Expression {
    pub fn parse(input: String) -> Self {
        let tokens = lex(input);



        Self {}
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum LexItem {
    LeftParenthesis,
    RightParenthesis,
    Space,
    EqualSign,
    LessThanSign,
    GreaterThanSign,
    DoubleQuote,
    SingleQuote,
    Char(char),
    SemiColon,
    Comma,
    ExclamationPoint,
}
impl LexItem {
    pub fn parse_char(c: char) -> Result<Self, LexError> {
        use LexItem::*;
        match c {
            '(' => Ok(LeftParenthesis),
            ')' => Ok(RightParenthesis),
            ' ' => Ok(Space),
            '=' => Ok(EqualSign),
            '<' => Ok(LessThanSign),
            '>' => Ok(GreaterThanSign),
            'A'..='z' => Ok(Char(c)),
            '0'..='9' => Ok(Char(c)),
            ';' => Ok(SemiColon),
            ',' => Ok(Comma),
            '\"' => Ok(DoubleQuote),
            '\'' => Ok(SingleQuote),
            '!' => Ok(ExclamationPoint),
            _ => Err(LexError::InvalidCharacter(c)),
        }
    }
    pub fn is_char(&self) -> bool {
        if let Self::Char(_) = self {
            true
        } else {
            false
        }
    }

    pub fn to_char(&self) -> char {
        use LexItem::*;
        match self {
            LeftParenthesis => '(',
            RightParenthesis => ')',
            Space => ' ',
            EqualSign => '=',
            LessThanSign => '<',
            GreaterThanSign => '>',
            DoubleQuote => '\"',
            SingleQuote => '\'',
            Char(c) => *c,
            SemiColon => ';',
            Comma => ',',
            ExclamationPoint => '!',
        }
    }
}

#[derive(Debug)]
pub enum LexError {
    InvalidCharacter(char),
}


pub fn lex(input: String) -> Result<Vec<LexItem>, LexError> {
    input.chars().map(|c| LexItem::parse_char(c)).collect()
}


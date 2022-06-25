use std::{error::Error, fmt::Display, num::ParseIntError, str::Chars};

use self::Token::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LexError {
    /// End of file
    Eof,
    Expected {
        expected: char,
        got: char,
    },
    InvalidIdentifier,
    UnrecognizedChar(char),
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for LexError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::Eof => write!(f, "End of file"),
            LexError::Expected { expected, got } => {
                write!(f, "Expected '{}', got '{}'", expected, got)
            }
            LexError::InvalidIdentifier => write!(f, "Identifier must not end with _"),
            LexError::UnrecognizedChar(c) => write!(f, "Char '{}' unrecognized", c),
            LexError::ParseInt(e) => write!(f, "{}", e),
        }
    }
}

impl Error for LexError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// "-42", "55"
    Int(i32),
    /// "true", "false"
    Bool(bool),
    // /// "{ac_foo_bar}"
    Ident(String),
    /// "UTF-8 encoded string. :)"
    Text(String),

    /// ","
    Comma,
    /// "("
    OpenParen,
    /// ")"
    CloseParen,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,
    /// "=="
    Eq,
    /// "!="
    Neq,
    /// "!"
    Not,
    /// "<"
    Lt,
    /// "<="
    Le,
    /// ">"
    Gt,
    /// ">="
    Ge,
    /// "and"
    And,
    /// "or"
    Or,
    /// "%"
    Percent,
    /// "in"
    In,
}

struct Cursor<'a> {
    initial_len: usize,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            initial_len: input.len(),
            chars: input.chars(),
        }
    }

    fn len_consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_second(&mut self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next()
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        // Predicate does not implement Copy so it was moved in the previous iteration.
        #[allow(clippy::redundant_closure)]
        while self.peek().map(|c| predicate(c)).unwrap_or(false) {
            self.bump();
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: Cursor::new(input),
        }
    }

    fn text_literal(&mut self) -> Result<Token, LexError> {
        let start = self.cursor.len_consumed();
        self.cursor.eat_while(|c| c != '\'');
        let stop = self.cursor.len_consumed();
        self.cursor.bump().ok_or(LexError::Eof)?;
        Ok(Text(self.input[start..stop].into()))
    }

    fn integer_literal(&mut self) -> Result<Token, LexError> {
        let start = self.cursor.len_consumed() - 1;
        self.cursor.eat_while(|c| c.is_digit(10));
        let stop = self.cursor.len_consumed();
        Ok(Int(self.input[start..stop].parse()?))
    }

    fn identifier(&mut self) -> Result<&str, LexError> {
        let valid_ident_char = |c: char| c.is_alphanumeric() || c == '_';
        let start = self.cursor.len_consumed() - 1;
        while self.cursor.peek().map(valid_ident_char).unwrap_or(false) {
            if self.cursor.peek() == Some('_')
                && !self
                    .cursor
                    .peek_second()
                    .map(valid_ident_char)
                    .unwrap_or(false)
            {
                return Err(LexError::InvalidIdentifier);
            }
            self.cursor.bump();
        }
        let stop = self.cursor.len_consumed();
        Ok(&self.input[start..stop])
    }

    fn advance_token(&mut self) -> Result<Token, LexError> {
        self.cursor.eat_while(|c| c.is_whitespace());
        match self.cursor.bump().ok_or(LexError::Eof)? {
            ',' => Ok(Comma),
            '(' => Ok(OpenParen),
            ')' => Ok(CloseParen),
            '[' => Ok(OpenBracket),
            ']' => Ok(CloseBracket),
            '!' => match self.cursor.peek().ok_or(LexError::Eof)? {
                '=' => {
                    self.cursor.bump();
                    Ok(Neq)
                }
                _ => Ok(Not),
            },
            '%' => Ok(Percent),
            '=' => match self.cursor.bump().ok_or(LexError::Eof)? {
                '=' => Ok(Eq),
                c => Err(LexError::Expected {
                    expected: '=',
                    got: c,
                }),
            },
            '<' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.bump();
                    Ok(Le)
                }
                _ => Ok(Lt),
            },
            '>' => match self.cursor.peek() {
                Some('=') => {
                    self.cursor.bump();
                    Ok(Ge)
                }
                _ => Ok(Gt),
            },
            '\'' => self.text_literal(),
            '-' | '0'..='9' => self.integer_literal(),
            'a'..='z' => match self.identifier()? {
                "and" => Ok(And),
                "or" => Ok(Or),
                "true" => Ok(Bool(true)),
                "false" => Ok(Bool(false)),
                "in" => Ok(In),
                ident => Ok(Ident(ident.into())),
            },
            c => Err(LexError::UnrecognizedChar(c)),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_token().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_lit() {
        assert_eq!(Ok(Bool(true)), Lexer::new("true").advance_token());
        assert_eq!(Some(Bool(false)), Lexer::new("false").next());
        assert_eq!(Some(Int(-42)), Lexer::new("-42").next());
        assert_eq!(Some(Int(55)), Lexer::new("55").next());
        println!("{:?}", Lexer::new("'Hello World! :)'").next().unwrap());
        assert_eq!(
            Some(Text("Hello World! :)".into())),
            Lexer::new("'Hello World! :)'").next()
        );
    }

    #[test]
    fn lex_ident_partial() {
        let mut lex = Lexer::new("te.st");
        assert_eq!(Ok(Ident("te".into())), lex.advance_token());
        assert_eq!(Err(LexError::UnrecognizedChar('.')), lex.advance_token());
    }

    #[test]
    fn lex_ident_endswith_underscore() {
        let mut lex = Lexer::new("test_");
        assert_eq!(Err(LexError::InvalidIdentifier), lex.advance_token());
    }

    #[test]
    fn very_long_int() {
        let input = "-100000000000000000000000000000000000000000000";
        let mut lex = Lexer::new(input);
        assert_eq!(None, lex.next());
    }

    #[test]
    fn parens() {
        let mut lex = Lexer::new("(((-42)))");
        assert_eq!(Some(OpenParen), lex.next());
        assert_eq!(Some(OpenParen), lex.next());
        assert_eq!(Some(OpenParen), lex.next());
        assert_eq!(Some(Int(-42)), lex.next());
        assert_eq!(Some(CloseParen), lex.next());
        assert_eq!(Some(CloseParen), lex.next());
        assert_eq!(Some(CloseParen), lex.next());
    }

    #[test]
    fn bool_negation() {
        let mut lex = Lexer::new("!true");
        assert_eq!(Some(Not), lex.next());
        assert_eq!(Some(Bool(true)), lex.next());
    }

    #[test]
    fn lex_multiple() {
        let input = "dep == 'EDDF' and sidwpt in ['TOBAK', 'ANEKI'] and iseven(rfl)";
        let mut lex = Lexer::new(input);
        assert_eq!(Some(Ident("dep".into())), lex.next());
        assert_eq!(Some(Eq), lex.next());
        assert_eq!(Some(Text("EDDF".into())), lex.next());
        assert_eq!(Some(And), lex.next());
        assert_eq!(Some(Ident("sidwpt".into())), lex.next());
        assert_eq!(Some(In), lex.next());
        assert_eq!(Some(OpenBracket), lex.next());
        assert_eq!(Some(Text("TOBAK".into())), lex.next());
        assert_eq!(Some(Comma), lex.next());
        assert_eq!(Some(Text("ANEKI".into())), lex.next());
        assert_eq!(Some(CloseBracket), lex.next());
        assert_eq!(Some(And), lex.next());
        assert_eq!(Some(Ident("iseven".into())), lex.next());
        assert_eq!(Some(OpenParen), lex.next());
        assert_eq!(Some(Ident("rfl".into())), lex.next());
        assert_eq!(Some(CloseParen), lex.next());
        assert_eq!(None, lex.next());
    }

    #[test]
    fn comparison_tokens() {
        let input = "! == != < <= > >=";
        let mut lex = Lexer::new(input);
        assert_eq!(Some(Not), lex.next());
        assert_eq!(Some(Eq), lex.next());
        assert_eq!(Some(Neq), lex.next());
        assert_eq!(Some(Lt), lex.next());
        assert_eq!(Some(Le), lex.next());
        assert_eq!(Some(Gt), lex.next());
        assert_eq!(Some(Ge), lex.next());
    }

    #[test]
    fn lex_peek() {
        let input = "rfl % 2000 == 0";
        let lex = Lexer::new(input);
        let mut lex = lex.peekable();
        assert_eq!(Some(Ident("rfl".into())), lex.next());
        assert_eq!(Some(&Percent), lex.peek());
        assert_eq!(Some(Percent), lex.next());
        assert_eq!(Some(Int(2000)), lex.next());
    }
}

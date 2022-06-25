use std::{error::Error, fmt::Display, iter::Peekable};

use super::{
    ast::{BinOp, Expr, Lit, UnOp},
    lexer::{Lexer, Token},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    PrematureEof,
    UnmatchedParen,
    UnmatchedBracked,
    BadToken(Token),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::PrematureEof => write!(f, "Got EOF but expected further token"),
            ParseError::UnmatchedParen => write!(f, "Parenthesis unmatched, expected ')'"),
            ParseError::UnmatchedBracked => write!(f, "Bracket unmatched, expected ']'"),
            ParseError::BadToken(t) => write!(f, "Bad token \"{:?}\"", t),
        }
    }
}

impl Error for ParseError {}

pub fn parse<S: AsRef<str>>(input: S) -> Result<Expr, ParseError> {
    let input = input.as_ref();
    let lexer = Lexer::new(input);
    let mut token_stream = lexer.peekable();
    expr_bp(&mut token_stream, 0)
}

fn parse_lhs(lexer: &mut Peekable<Lexer<'_>>) -> Result<Expr, ParseError> {
    match lexer.next().ok_or(ParseError::PrematureEof)? {
        Token::Bool(b) => Ok(Expr::Lit(Lit::Bool(b))),
        Token::Int(i) => Ok(Expr::Lit(Lit::Int(i.into()))),
        Token::Text(s) => Ok(Expr::Lit(Lit::Text(s))),
        Token::Ident(id) => Ok(Expr::Ident(id)),
        Token::OpenParen => {
            let lhs = expr_bp(lexer, 0)?;
            if lexer.next() != Some(Token::CloseParen) {
                return Err(ParseError::UnmatchedParen);
            }
            Ok(lhs)
        }
        Token::OpenBracket => {
            let mut exprs = vec![expr_bp(lexer, 0)?];
            while let Some(Token::Comma) = lexer.peek() {
                lexer.next();
                exprs.push(expr_bp(lexer, 0)?);
            }
            if lexer.next() != Some(Token::CloseBracket) {
                return Err(ParseError::UnmatchedBracked);
            }
            Ok(Expr::Array(exprs))
        }
        Token::Not => {
            let ((), r_bp) = prefix_binding_power(&UnOp::Not);
            let rhs = expr_bp(lexer, r_bp)?;
            Ok(Expr::Unary(UnOp::Not, Box::new(rhs)))
        }
        t => Err(ParseError::BadToken(t)),
    }
}

fn expr_bp(lexer: &mut Peekable<Lexer<'_>>, min_bp: u8) -> Result<Expr, ParseError> {
    let mut lhs = parse_lhs(lexer)?;

    loop {
        let token = match lexer.peek() {
            None => break,
            Some(t) => t,
        };

        // Postfix could come here

        let maybe_binop = match token {
            Token::And => Some(BinOp::And),
            Token::Or => Some(BinOp::Or),
            Token::Eq => Some(BinOp::Eq),
            Token::Neq => Some(BinOp::Neq),
            Token::Ge => Some(BinOp::Ge),
            Token::Gt => Some(BinOp::Gt),
            Token::Le => Some(BinOp::Le),
            Token::Lt => Some(BinOp::Lt),
            Token::Percent => Some(BinOp::Mod),
            Token::In => Some(BinOp::In),
            _ => None,
        };
        if let Some(op) = maybe_binop {
            let (l_bp, r_bp) = infix_binding_power(&op);
            if l_bp < min_bp {
                break;
            }
            lexer.next();

            let rhs = expr_bp(lexer, r_bp)?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
            continue;
        }

        break;
    }

    Ok(lhs)
}

fn prefix_binding_power(op: &UnOp) -> ((), u8) {
    match op {
        UnOp::Not => ((), 9),
    }
}

fn infix_binding_power(op: &BinOp) -> (u8, u8) {
    match op {
        BinOp::Or => (1, 2),
        BinOp::And => (3, 4),
        BinOp::In => (5, 6),
        BinOp::Eq | BinOp::Neq | BinOp::Ge | BinOp::Gt | BinOp::Le | BinOp::Lt => (7, 8),
        BinOp::Mod => (9, 10),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_lit() {
        assert_eq!(Ok(Expr::Lit(Lit::Bool(true))), parse("true"));
    }

    #[test]
    fn int() {
        assert_eq!(Ok(Expr::Lit(Lit::Int(55))), parse("55"));
        assert_eq!(Ok(Expr::Lit(Lit::Int(-42))), parse("-42"));
    }

    #[test]
    fn paren_int() {
        assert_eq!(Ok(Expr::Lit(Lit::Int(-42))), parse("(((-42)))"));
    }

    #[test]
    fn ident() {
        assert_eq!(Ok(Expr::Ident("dep".into())), parse("dep"));
        assert_eq!(Ok(Expr::Ident("ac_typ".into())), parse("ac_typ"));
        assert_eq!(
            Ok(Expr::Ident("ac_faa_equip_code".into())),
            parse("ac_faa_equip_code")
        );
        assert!(parse("_test").is_err());
        assert!(parse("test_").is_err());
        assert!(parse("test_test").is_ok());
        assert!(parse("ac_test_test").is_ok());
    }

    #[test]
    fn array() {
        let expected = Expr::Array(vec![
            Expr::Lit(Lit::Int(42)),
            Expr::Unary(UnOp::Not, Box::new(Expr::Lit(Lit::Bool(true)))),
            Expr::Lit(Lit::Text("ANEKI".into())),
        ]);
        assert_eq!(Ok(expected), parse("[42, !true, 'ANEKI']"))
    }

    #[test]
    fn precedence() {
        let expected = Expr::Binary(
            BinOp::Or,
            Box::new(Expr::Binary(
                BinOp::And,
                Box::new(Expr::Binary(
                    BinOp::Eq,
                    Box::new(Expr::Unary(UnOp::Not, Box::new(Expr::Ident("rvsm".into())))),
                    Box::new(Expr::Lit(Lit::Bool(true))),
                )),
                Box::new(Expr::Binary(
                    BinOp::Neq,
                    Box::new(Expr::Binary(
                        BinOp::Mod,
                        Box::new(Expr::Ident("rfl".into())),
                        Box::new(Expr::Lit(Lit::Int(2000))),
                    )),
                    Box::new(Expr::Lit(Lit::Int(0))),
                )),
            )),
            Box::new(Expr::Binary(
                BinOp::In,
                Box::new(Expr::Ident("sid".into())),
                Box::new(Expr::Lit(Lit::Text("ANEKI1L".into()))),
            )),
        );
        assert_eq!(
            Ok(expected),
            parse("!rvsm == true and rfl % 2000 != 0 or sid in 'ANEKI1L'")
        );
    }

    #[test]
    fn complex() {
        let expected_expr = Expr::Binary(
            BinOp::Neq,
            Box::new(Expr::Lit(Lit::Bool(true))),
            Box::new(Expr::Binary(
                BinOp::Eq,
                Box::new(Expr::Binary(
                    BinOp::Neq,
                    Box::new(Expr::Lit(Lit::Text("test".into()))),
                    Box::new(Expr::Lit(Lit::Text("notest".into()))),
                )),
                Box::new(Expr::Binary(
                    BinOp::And,
                    Box::new(Expr::Binary(
                        BinOp::Lt,
                        Box::new(Expr::Lit(Lit::Int(123))),
                        Box::new(Expr::Lit(Lit::Int(10))),
                    )),
                    Box::new(Expr::Unary(UnOp::Not, Box::new(Expr::Lit(Lit::Bool(true))))),
                )),
            )),
        );
        let parsed = parse("true != (('test' != 'notest') == ((123 < 10) and (!true)))").unwrap();

        assert_eq!(expected_expr, parsed);
    }
}

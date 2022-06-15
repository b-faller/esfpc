use pest::Parser;

use crate::ast;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct AstParser;

fn build_unary(pair: pest::iterators::Pair<Rule>) -> ast::Expr {
    let mut pair = pair.into_inner();
    let unop = match pair.next().expect("unary has operator").as_str() {
        "!" => ast::UnOp::Not,
        _ => unreachable!(),
    };
    let expr = pair.next().expect("unary has expression");
    ast::Expr::new(ast::ExprKind::Unary(unop, Box::new(build_expr(expr))))
}

fn build_binary(pair: pest::iterators::Pair<Rule>) -> ast::Expr {
    let mut pair = pair.into_inner();
    let left_expr = pair.next().expect("binary has left expression");
    let binop = match pair.next().expect("binary has operator").as_str() {
        "&&" => ast::BinOp::And,
        "||" => ast::BinOp::Or,
        "==" => ast::BinOp::Eq,
        "!=" => ast::BinOp::Neq,
        ">=" => ast::BinOp::Ge,
        ">" => ast::BinOp::Gt,
        "<=" => ast::BinOp::Le,
        "<" => ast::BinOp::Lt,
        "%" => ast::BinOp::Mod,
        _ => unreachable!(),
    };
    let right_expr = pair.next().expect("binary has right expression");
    ast::Expr::new(ast::ExprKind::Binary(
        binop,
        Box::new(build_expr(left_expr)),
        Box::new(build_expr(right_expr)),
    ))
}

fn build_lit(pair: pest::iterators::Pair<Rule>) -> ast::Expr {
    let pair = pair.into_inner().next().expect("literal has value");
    let lit_kind = match pair.as_rule() {
        Rule::bool => ast::LitKind::Bool(pair.as_str().parse().expect("bool is parsable")),
        Rule::int => ast::LitKind::Int(pair.as_str().parse().expect("int is parsable")),
        Rule::str => ast::LitKind::String(pair.as_str().parse().expect("string is parsable")),
        Rule::var => ast::LitKind::Var(pair.as_str().parse().expect("variable is parsable")),
        _ => unreachable!(),
    };
    ast::Expr::new(ast::ExprKind::Lit(lit_kind))
}

fn build_expr(pair: pest::iterators::Pair<Rule>) -> ast::Expr {
    match pair.as_rule() {
        Rule::Lit => build_lit(pair),
        Rule::Unary => build_unary(pair),
        Rule::Binary => build_binary(pair),
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}
pub fn parse<S: AsRef<str>>(src: S) -> Result<ast::Expr, pest::error::Error<Rule>> {
    let ast = AstParser::parse(Rule::ExprStr, src.as_ref())?
        .next()
        .expect("parser matches one expression");
    let expr = build_expr(ast);
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lit() {
        assert_eq!(
            Ok(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::Bool(true)))),
            parse("true")
        )
    }

    #[test]
    fn parse_var() {
        assert_eq!(
            Ok(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::Var(
                "dep".into()
            )))),
            parse("dep")
        );
        assert_eq!(
            Ok(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::Var(
                "ac.typ".into()
            )))),
            parse("ac.typ")
        );
        assert_eq!(
            Ok(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::Var(
                "ac.faa_equip_code".into()
            )))),
            parse("ac.faa_equip_code")
        );
        assert!(parse("_test").is_err());
        assert!(parse("test_").is_err());
        assert!(parse("test_test").is_ok());
        assert!(parse("test__test").is_err());
        assert!(parse("ac._test").is_err());
        assert!(parse("ac.test_").is_err());
        assert!(parse("ac.test_.test").is_err());
        assert!(parse("ac.test.test").is_ok());
    }

    #[test]
    fn test_complex() {
        let expected_expr = ast::Expr::new(ast::ExprKind::Binary(
            ast::BinOp::Neq,
            Box::new(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::Bool(true)))),
            Box::new(ast::Expr::new(ast::ExprKind::Binary(
                ast::BinOp::Eq,
                Box::new(ast::Expr::new(ast::ExprKind::Binary(
                    ast::BinOp::Neq,
                    Box::new(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::String(
                        "test".into(),
                    )))),
                    Box::new(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::String(
                        "notest".into(),
                    )))),
                ))),
                Box::new(ast::Expr::new(ast::ExprKind::Binary(
                    ast::BinOp::And,
                    Box::new(ast::Expr::new(ast::ExprKind::Binary(
                        ast::BinOp::Lt,
                        Box::new(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::Int(123)))),
                        Box::new(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::Int(10)))),
                    ))),
                    Box::new(ast::Expr::new(ast::ExprKind::Unary(
                        ast::UnOp::Not,
                        Box::new(ast::Expr::new(ast::ExprKind::Lit(ast::LitKind::Bool(true)))),
                    ))),
                ))),
            ))),
        ));
        let parsed = parse("true != (('test' != 'notest') == ((123 < 10) && (!true)))").unwrap();

        assert_eq!(expected_expr, parsed);
    }
}

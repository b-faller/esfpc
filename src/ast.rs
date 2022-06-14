use crate::ffi;

#[derive(Debug, PartialEq, Eq)]
pub struct Expr {
    kind: ExprKind,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExprKind {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Lit(LitKind),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinOp {
    And,
    Or,
    Eq,
    Neq,
    Ge,
    Gt,
    Le,
    Lt,
    Mod,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnOp {
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LitKind {
    Bool(bool),
    Int(i32),
    String(String),
    Var(String),
}

fn and_op(l: LitKind, r: LitKind) -> Result<LitKind, &'static str> {
    match (l, r) {
        (LitKind::Bool(lv), LitKind::Bool(rv)) => Ok(LitKind::Bool(lv && rv)),
        _ => Err("Can only and bool"),
    }
}

fn or_op(l: LitKind, r: LitKind) -> Result<LitKind, &'static str> {
    match (l, r) {
        (LitKind::Bool(lv), LitKind::Bool(rv)) => Ok(LitKind::Bool(lv || rv)),
        _ => Err("Can only and bool"),
    }
}

pub fn eval(expr: &Expr, fp: &ffi::FlightPlan) -> Result<LitKind, &'static str> {
    match &expr.kind {
        ExprKind::Binary(op, left_expr, right_expr) => {
            let l = eval(left_expr, fp)?;
            let r = eval(right_expr, fp)?;

            match op {
                BinOp::And => and_op(l, r),
                BinOp::Or => or_op(l, r),
                BinOp::Eq => Ok(LitKind::Bool(l == r)),
                BinOp::Neq => Ok(LitKind::Bool(l != r)),
                inequation => match (l, r) {
                    (LitKind::Int(l), LitKind::Int(r)) => match inequation {
                        BinOp::Ge => Ok(LitKind::Bool(l >= r)),
                        BinOp::Gt => Ok(LitKind::Bool(l > r)),
                        BinOp::Le => Ok(LitKind::Bool(l <= r)),
                        BinOp::Lt => Ok(LitKind::Bool(l < r)),
                        BinOp::Mod => Ok(LitKind::Int(l % r)),
                        _ => Err("Cannot compare inequalness with non integer type"),
                    },
                    _ => unreachable!(),
                },
            }
        }
        ExprKind::Unary(op, expr) => match op {
            UnOp::Not => match eval(expr, fp)? {
                LitKind::Bool(v) => Ok(LitKind::Bool(!v)),
                _ => Err("Cannot negate this literal"),
            },
        },
        ExprKind::Lit(kind) => match kind {
            LitKind::Var(var) => match var.as_str() {
                "rule" => Ok(LitKind::String(fp.rule.to_string())),
                "rfl" => Ok(LitKind::Int(fp.rfl)),
                "adep" => Ok(LitKind::String(fp.adep.clone())),
                "adest" => Ok(LitKind::String(fp.adest.clone())),
                "sid" => Ok(LitKind::String(fp.sid.clone())),
                _ => Err("variable is not implemented"),
            },
            _ => Ok(kind.clone()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for ffi::FlightPlan {
        fn default() -> Self {
            Self {
                rule: ffi::FlightRule::Ifr,
                rfl: 35000,
                adep: "EDDF".to_string(),
                adest: "EDDM".to_string(),
                sid: "CINDY4S".to_string(),
            }
        }
    }

    #[test]
    fn simple_expr() {
        let expr = Expr {
            kind: ExprKind::Unary(
                UnOp::Not,
                Box::new(Expr {
                    kind: ExprKind::Binary(
                        BinOp::Eq,
                        Box::new(Expr {
                            kind: ExprKind::Lit(LitKind::Bool(false)),
                        }),
                        Box::new(Expr {
                            kind: ExprKind::Lit(LitKind::Bool(true)),
                        }),
                    ),
                }),
            ),
        };
        assert_eq!(
            Ok(LitKind::Bool(true)),
            eval(&expr, &ffi::FlightPlan::default())
        )
    }

    #[test]
    fn invalid_not_int() {
        let expr = Expr {
            kind: ExprKind::Unary(
                UnOp::Not,
                Box::new(Expr {
                    kind: ExprKind::Lit(LitKind::Int(42)),
                }),
            ),
        };
        assert!(eval(&expr, &ffi::FlightPlan::default()).is_err())
    }

    #[test]
    fn str_eq() {
        let expr = Expr {
            kind: ExprKind::Binary(
                BinOp::Eq,
                Box::new(Expr {
                    kind: ExprKind::Lit(LitKind::String("Hello World!".into())),
                }),
                Box::new(Expr {
                    kind: ExprKind::Lit(LitKind::String("Hello World!".into())),
                }),
            ),
        };
        assert_eq!(
            Ok(LitKind::Bool(true)),
            eval(&expr, &ffi::FlightPlan::default())
        )
    }

    #[test]
    fn fp_vars() {
        let fp = ffi::FlightPlan {
            rule: ffi::FlightRule::Ifr,
            rfl: 35000,
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Binary(
            BinOp::Eq,
            Box::new(Expr::new(ExprKind::Lit(LitKind::Var("rfl".into())))),
            Box::new(Expr::new(ExprKind::Lit(LitKind::Int(35000)))),
        ));
        assert_eq!(Ok(LitKind::Bool(true)), eval(&expr, &fp))
    }

    #[test]
    fn var_comparison() {
        let fp = ffi::FlightPlan {
            rfl: 35000,
            ..Default::default()
        };

        let expr = Expr::new(ExprKind::Binary(
            BinOp::Le,
            Box::new(Expr::new(ExprKind::Lit(LitKind::Var("rfl".into())))),
            Box::new(Expr::new(ExprKind::Lit(LitKind::Int(35000)))),
        ));
        assert_eq!(Ok(LitKind::Bool(true)), eval(&expr, &fp));

        let expr = Expr::new(ExprKind::Binary(
            BinOp::Le,
            Box::new(Expr::new(ExprKind::Lit(LitKind::Var("rfl".into())))),
            Box::new(Expr::new(ExprKind::Lit(LitKind::Int(34999)))),
        ));
        assert_eq!(Ok(LitKind::Bool(false)), eval(&expr, &fp));

        let expr = Expr::new(ExprKind::Binary(
            BinOp::Le,
            Box::new(Expr::new(ExprKind::Lit(LitKind::Var("rfl".into())))),
            Box::new(Expr::new(ExprKind::Lit(LitKind::Int(35001)))),
        ));
        assert_eq!(Ok(LitKind::Bool(true)), eval(&expr, &fp));
    }
}

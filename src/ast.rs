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
    Int(i64),
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

fn var(var: &str, fp: &ffi::FlightPlan) -> Result<LitKind, &'static str> {
    match var {
        "ac.typ" => Ok(LitKind::String(fp.ac.typ.to_string())),
        "ac.wtc" => Ok(LitKind::String(fp.ac.wtc.to_string())),
        "ac.faa_equip_code" => Ok(LitKind::String(fp.ac.faa_equip_code.to_string())),
        "ac.eng_typ" => Ok(LitKind::String(fp.ac.eng_typ.to_string())),
        "ac.eng_count" => Ok(LitKind::Int(fp.ac.eng_count.into())),
        "ac.is_rvsm_capable" => Ok(LitKind::Bool(fp.ac.is_rvsm_capable)),
        "rule" => Ok(LitKind::String(fp.rule.to_string())),
        "cfl" => Ok(LitKind::Int(fp.cfl.into())),
        "rfl" => Ok(LitKind::Int(fp.rfl.into())),
        "dep" => Ok(LitKind::String(fp.dep.clone())),
        "dep_rwy" => Ok(LitKind::String(fp.dep_rwy.clone())),
        "arr" => Ok(LitKind::String(fp.arr.clone())),
        "sid" => Ok(LitKind::String(fp.sid.clone())),
        "route" => Ok(LitKind::String(fp.route.clone())),
        _ => Err("variable is not implemented"),
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
            LitKind::Var(v) => var(v.as_str(), fp),
            _ => Ok(kind.clone()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Default for ffi::Aircraft {
        fn default() -> Self {
            Self {
                typ: ffi::AircraftType::Landplane,
                wtc: ffi::WakeTurbulenceCategory::Medium,
                faa_equip_code: ffi::FaaEquipmentCode::Q,
                eng_typ: ffi::EngineType::Jet,
                eng_count: 2,
                is_rvsm_capable: true,
            }
        }
    }

    impl Default for ffi::FlightPlan {
        fn default() -> Self {
            Self {
                ac: ffi::Aircraft::default(),
                rule: ffi::FlightRule::Ifr,
                cfl: 4000,
                rfl: 35000,
                dep: "EDDF".to_string(),
                dep_rwy: "18".to_string(),
                arr: "EDDM".to_string(),
                sid: "CINDY4S".to_string(),
                route: "CINDY Z74 HAREM T104 ROKIL".to_string(),
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
    fn var_int_comparison() {
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

    #[test]
    fn var_ac_eval() {
        let fp = ffi::FlightPlan {
            ac: ffi::Aircraft {
                typ: ffi::AircraftType::Helicopter,
                wtc: ffi::WakeTurbulenceCategory::Light,
                faa_equip_code: ffi::FaaEquipmentCode::G,
                eng_typ: ffi::EngineType::Electric,
                eng_count: 1,
                is_rvsm_capable: false,
            },
            ..Default::default()
        };

        let expr = Expr::new(ExprKind::Lit(LitKind::Var("ac.typ".into())));
        assert_eq!(Ok(LitKind::String("H".into())), eval(&expr, &fp));

        let expr = Expr::new(ExprKind::Lit(LitKind::Var("ac.wtc".into())));
        assert_eq!(Ok(LitKind::String("L".into())), eval(&expr, &fp));

        let expr = Expr::new(ExprKind::Lit(LitKind::Var("ac.faa_equip_code".into())));
        assert_eq!(Ok(LitKind::String("G".into())), eval(&expr, &fp));

        let expr = Expr::new(ExprKind::Lit(LitKind::Var("ac.eng_typ".into())));
        assert_eq!(Ok(LitKind::String("E".into())), eval(&expr, &fp));

        let expr = Expr::new(ExprKind::Lit(LitKind::Var("ac.eng_count".into())));
        assert_eq!(Ok(LitKind::Int(1)), eval(&expr, &fp));

        let expr = Expr::new(ExprKind::Lit(LitKind::Var("ac.is_rvsm_capable".into())));
        assert_eq!(Ok(LitKind::Bool(false)), eval(&expr, &fp));
    }

    #[test]
    fn var_rule_eval() {
        let fp = ffi::FlightPlan {
            rule: ffi::FlightRule::Zulu,
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Lit(LitKind::Var("rule".into())));
        assert_eq!(Ok(LitKind::String("Z".into())), eval(&expr, &fp))
    }

    #[test]
    fn var_cfl_eval() {
        let fp = ffi::FlightPlan {
            cfl: 4000,
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Lit(LitKind::Var("cfl".into())));
        assert_eq!(Ok(LitKind::Int(4000)), eval(&expr, &fp))
    }

    #[test]
    fn var_rfl_eval() {
        let fp = ffi::FlightPlan {
            rfl: 35000,
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Lit(LitKind::Var("rfl".into())));
        assert_eq!(Ok(LitKind::Int(35000)), eval(&expr, &fp))
    }

    #[test]
    fn var_dep_eval() {
        let fp = ffi::FlightPlan {
            dep: "EDDF".to_string(),
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Lit(LitKind::Var("dep".into())));
        assert_eq!(Ok(LitKind::String("EDDF".into())), eval(&expr, &fp))
    }

    #[test]
    fn var_dep_rwy_eval() {
        let fp = ffi::FlightPlan {
            dep_rwy: "07C".to_string(),
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Lit(LitKind::Var("dep_rwy".into())));
        assert_eq!(Ok(LitKind::String("07C".into())), eval(&expr, &fp))
    }

    #[test]
    fn var_arr_eval() {
        let fp = ffi::FlightPlan {
            arr: "EDDS".to_string(),
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Lit(LitKind::Var("arr".into())));
        assert_eq!(Ok(LitKind::String("EDDS".into())), eval(&expr, &fp))
    }

    #[test]
    fn var_sid_eval() {
        let fp = ffi::FlightPlan {
            sid: "CINDY4S".to_string(),
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Lit(LitKind::Var("sid".into())));
        assert_eq!(Ok(LitKind::String("CINDY4S".into())), eval(&expr, &fp))
    }

    #[test]
    fn var_route_eval() {
        let fp = ffi::FlightPlan {
            route: "CINDY Z74 HAREM T104 ROKIL".to_string(),
            ..Default::default()
        };
        let expr = Expr::new(ExprKind::Lit(LitKind::Var("route".into())));
        assert_eq!(
            Ok(LitKind::String("CINDY Z74 HAREM T104 ROKIL".into())),
            eval(&expr, &fp)
        )
    }
}

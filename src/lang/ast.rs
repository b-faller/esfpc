use crate::ffi;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Lit(Lit),
    Ident(String),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Array(Vec<Expr>),
    // Call(Box<Expr>, Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    In,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lit {
    Bool(bool),
    Int(i64),
    Text(String),
}

fn ident(ident: &str, fp: &ffi::FlightPlan) -> Result<Lit, &'static str> {
    match ident {
        "ac_type" => Ok(Lit::Text(fp.ac.typ.to_string())),
        "ac_wtc" => Ok(Lit::Text(fp.ac.wtc.to_string())),
        "ac_faa_equip_code" => Ok(Lit::Text(fp.ac.faa_equip_code.to_string())),
        "rnav" => Ok(Lit::Bool(fp.ac.faa_equip_code.is_rnav())),
        "ac_eng_type" => Ok(Lit::Text(fp.ac.eng_typ.to_string())),
        "ac_eng_count" => Ok(Lit::Int(fp.ac.eng_count.into())),
        "ac_is_rvsm_capable" => Ok(Lit::Bool(fp.ac.is_rvsm_capable)),
        "rule" => Ok(Lit::Text(fp.rule.to_string())),
        "cfl" => Ok(Lit::Int(fp.cfl.into())),
        "rfl" => Ok(Lit::Int(fp.rfl.into())),
        "dep" => Ok(Lit::Text(fp.dep.clone())),
        "dep_rwy" => Ok(Lit::Text(fp.dep_rwy.clone())),
        "arr" => Ok(Lit::Text(fp.arr.clone())),
        "sid" => Ok(Lit::Text(fp.sid.clone())),
        "sidwpt" => Ok(Lit::Text(
            fp.sid
                .split(|c: char| c.is_ascii_digit())
                .next()
                .expect("Split returns always a str")
                .into(),
        )),
        "route" => Ok(Lit::Text(fp.route.clone())),
        _ => Err("Identifier is not implemented"),
    }
}

fn eval_inner(expr: &Expr, fp: &ffi::FlightPlan) -> Result<Expr, &'static str> {
    match &expr {
        Expr::Lit(lit) => Ok(Expr::Lit(lit.clone())),
        Expr::Ident(id) => Ok(Expr::Lit(ident(id, fp)?)),
        Expr::Binary(op, lhs, rhs) => {
            let lhs = eval_inner(lhs, fp)?;
            let rhs = eval_inner(rhs, fp)?;

            match (op, lhs, rhs) {
                (BinOp::And, Expr::Lit(Lit::Bool(lhs)), Expr::Lit(Lit::Bool(rhs))) => {
                    Ok(Expr::Lit(Lit::Bool(lhs && rhs)))
                }
                (BinOp::Or, Expr::Lit(Lit::Bool(lhs)), Expr::Lit(Lit::Bool(rhs))) => {
                    Ok(Expr::Lit(Lit::Bool(lhs || rhs)))
                }
                (BinOp::Eq, lhs, rhs) => Ok(Expr::Lit(Lit::Bool(lhs == rhs))),
                (BinOp::Neq, lhs, rhs) => Ok(Expr::Lit(Lit::Bool(lhs != rhs))),
                (BinOp::Lt, Expr::Lit(Lit::Int(lhs)), Expr::Lit(Lit::Int(rhs))) => {
                    Ok(Expr::Lit(Lit::Bool(lhs < rhs)))
                }
                (BinOp::Le, Expr::Lit(Lit::Int(lhs)), Expr::Lit(Lit::Int(rhs))) => {
                    Ok(Expr::Lit(Lit::Bool(lhs <= rhs)))
                }
                (BinOp::Gt, Expr::Lit(Lit::Int(lhs)), Expr::Lit(Lit::Int(rhs))) => {
                    Ok(Expr::Lit(Lit::Bool(lhs > rhs)))
                }
                (BinOp::Ge, Expr::Lit(Lit::Int(lhs)), Expr::Lit(Lit::Int(rhs))) => {
                    Ok(Expr::Lit(Lit::Bool(lhs >= rhs)))
                }
                (BinOp::Mod, Expr::Lit(Lit::Int(lhs)), Expr::Lit(Lit::Int(rhs))) => {
                    Ok(Expr::Lit(Lit::Int(lhs % rhs)))
                }
                (BinOp::In, item @ (Expr::Lit(_) | Expr::Array(_)), Expr::Array(exprs)) => {
                    Ok(Expr::Lit(Lit::Bool(exprs.contains(&item))))
                }
                (BinOp::In, Expr::Lit(Lit::Text(s1)), Expr::Lit(Lit::Text(s2))) => {
                    Ok(Expr::Lit(Lit::Bool(s2.contains(&s1))))
                }
                _ => Err("Invalid binary operation"),
            }
        }
        Expr::Unary(op, expr) => {
            let expr = eval_inner(expr, fp)?;
            match (op, expr) {
                (UnOp::Not, Expr::Lit(Lit::Bool(val))) => Ok(Expr::Lit(Lit::Bool(!val))),
                _ => Err("Cannot negate this expression"),
            }
        }
        Expr::Array(exprs) => {
            let mut evaluated_exprs = vec![];
            for expr in exprs {
                evaluated_exprs.push(eval_inner(expr, fp)?);
            }
            Ok(Expr::Array(evaluated_exprs))
        }
    }
}

pub fn eval_cond(expr: &Expr, fp: &ffi::FlightPlan) -> Result<bool, &'static str> {
    if let Expr::Lit(Lit::Bool(val)) = eval_inner(expr, fp)? {
        return Ok(val);
    }
    Err("Expression did not evaluate to true or false")
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
        let expr = Expr::Unary(
            UnOp::Not,
            Box::new(Expr::Binary(
                BinOp::Eq,
                Box::new(Expr::Lit(Lit::Bool(false))),
                Box::new(Expr::Lit(Lit::Bool(true))),
            )),
        );
        assert_eq!(Ok(true), eval_cond(&expr, &ffi::FlightPlan::default()))
    }

    #[test]
    fn invalid_not_int() {
        let expr = Expr::Unary(UnOp::Not, Box::new(Expr::Lit(Lit::Int(42))));
        assert!(eval_inner(&expr, &ffi::FlightPlan::default()).is_err())
    }

    #[test]
    fn in_array() {
        let expr = Expr::Binary(
            BinOp::In,
            Box::new(Expr::Lit(Lit::Text("TOBAK".into()))),
            Box::new(Expr::Array(vec![
                Expr::Lit(Lit::Text("ANEKI".into())),
                Expr::Lit(Lit::Text("TOBAK".into())),
            ])),
        );
        assert_eq!(Ok(true), eval_cond(&expr, &ffi::FlightPlan::default()));
        let expr = Expr::Binary(
            BinOp::In,
            Box::new(Expr::Lit(Lit::Text("OBOKA".into()))),
            Box::new(Expr::Array(vec![
                Expr::Lit(Lit::Text("ANEKI".into())),
                Expr::Lit(Lit::Text("TOBAK".into())),
            ])),
        );
        assert_eq!(Ok(false), eval_cond(&expr, &ffi::FlightPlan::default()));
    }

    #[test]
    fn in_text() {
        let expr = Expr::Binary(
            BinOp::In,
            Box::new(Expr::Lit(Lit::Text("TOBAK Z10".into()))),
            Box::new(Expr::Lit(Lit::Text(
                "TOBAK7M/25C TOBAK Z10 NOSEX DCT KLF".into(),
            ))),
        );
        assert_eq!(Ok(true), eval_cond(&expr, &ffi::FlightPlan::default()));
        let expr = Expr::Binary(
            BinOp::In,
            Box::new(Expr::Lit(Lit::Text("TOBAK Z10".into()))),
            Box::new(Expr::Lit(Lit::Text("Text".into()))),
        );
        assert_eq!(Ok(false), eval_cond(&expr, &ffi::FlightPlan::default()));
    }

    #[test]
    fn str_eq() {
        let expr = Expr::Binary(
            BinOp::Eq,
            Box::new(Expr::Lit(Lit::Text("Hello World!".into()))),
            Box::new(Expr::Lit(Lit::Text("Hello World!".into()))),
        );
        assert_eq!(Ok(true), eval_cond(&expr, &ffi::FlightPlan::default()))
    }

    #[test]
    fn fp_vars() {
        let fp = ffi::FlightPlan {
            rule: ffi::FlightRule::Ifr,
            rfl: 35000,
            ..Default::default()
        };
        let expr = Expr::Binary(
            BinOp::Eq,
            Box::new(Expr::Ident("rfl".into())),
            Box::new(Expr::Lit(Lit::Int(35000))),
        );

        assert_eq!(Ok(true), eval_cond(&expr, &fp))
    }

    #[test]
    fn var_int_comparison() {
        let fp = ffi::FlightPlan {
            rfl: 35000,
            ..Default::default()
        };

        let expr = Expr::Binary(
            BinOp::Le,
            Box::new(Expr::Ident("rfl".into())),
            Box::new(Expr::Lit(Lit::Int(35000))),
        );
        assert_eq!(Ok(true), eval_cond(&expr, &fp));

        let expr = Expr::Binary(
            BinOp::Le,
            Box::new(Expr::Ident("rfl".into())),
            Box::new(Expr::Lit(Lit::Int(34999))),
        );
        assert_eq!(Ok(false), eval_cond(&expr, &fp));

        let expr = Expr::Binary(
            BinOp::Le,
            Box::new(Expr::Ident("rfl".into())),
            Box::new(Expr::Lit(Lit::Int(35001))),
        );
        assert_eq!(Ok(true), eval_cond(&expr, &fp));
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

        let expr = Expr::Ident("ac_type".into());
        assert_eq!(Ok(Expr::Lit(Lit::Text("H".into()))), eval_inner(&expr, &fp));

        let expr = Expr::Ident("ac_wtc".into());
        assert_eq!(Ok(Expr::Lit(Lit::Text("L".into()))), eval_inner(&expr, &fp));

        let expr = Expr::Ident("ac_faa_equip_code".into());
        assert_eq!(Ok(Expr::Lit(Lit::Text("G".into()))), eval_inner(&expr, &fp));

        let expr = Expr::Ident("ac_eng_type".into());
        assert_eq!(Ok(Expr::Lit(Lit::Text("E".into()))), eval_inner(&expr, &fp));

        let expr = Expr::Ident("ac_eng_count".into());
        assert_eq!(Ok(Expr::Lit(Lit::Int(1))), eval_inner(&expr, &fp));

        let expr = Expr::Ident("ac_is_rvsm_capable".into());
        assert_eq!(Ok(Expr::Lit(Lit::Bool(false))), eval_inner(&expr, &fp));
    }

    #[test]
    fn var_rule_eval() {
        let fp = ffi::FlightPlan {
            rule: ffi::FlightRule::Zulu,
            ..Default::default()
        };
        let expr = Expr::Ident("rule".into());
        assert_eq!(Ok(Expr::Lit(Lit::Text("Z".into()))), eval_inner(&expr, &fp))
    }

    #[test]
    fn var_cfl_eval() {
        let fp = ffi::FlightPlan {
            cfl: 4000,
            ..Default::default()
        };
        let expr = Expr::Ident("cfl".into());
        assert_eq!(Ok(Expr::Lit(Lit::Int(4000))), eval_inner(&expr, &fp))
    }

    #[test]
    fn var_rfl_eval() {
        let fp = ffi::FlightPlan {
            rfl: 35000,
            ..Default::default()
        };
        let expr = Expr::Ident("rfl".into());
        assert_eq!(Ok(Expr::Lit(Lit::Int(35000))), eval_inner(&expr, &fp))
    }

    #[test]
    fn var_dep_eval() {
        let fp = ffi::FlightPlan {
            dep: "EDDF".to_string(),
            ..Default::default()
        };
        let expr = Expr::Ident("dep".into());
        assert_eq!(
            Ok(Expr::Lit(Lit::Text("EDDF".into()))),
            eval_inner(&expr, &fp)
        )
    }

    #[test]
    fn var_dep_rwy_eval() {
        let fp = ffi::FlightPlan {
            dep_rwy: "07C".to_string(),
            ..Default::default()
        };
        let expr = Expr::Ident("dep_rwy".into());
        assert_eq!(
            Ok(Expr::Lit(Lit::Text("07C".into()))),
            eval_inner(&expr, &fp)
        )
    }

    #[test]
    fn var_arr_eval() {
        let fp = ffi::FlightPlan {
            arr: "EDDS".to_string(),
            ..Default::default()
        };
        let expr = Expr::Ident("arr".into());
        assert_eq!(
            Ok(Expr::Lit(Lit::Text("EDDS".into()))),
            eval_inner(&expr, &fp)
        )
    }

    #[test]
    fn var_sid_eval() {
        let fp = ffi::FlightPlan {
            sid: "CINDY4S".to_string(),
            ..Default::default()
        };
        let expr = Expr::Ident("sid".into());
        assert_eq!(
            Ok(Expr::Lit(Lit::Text("CINDY4S".into()))),
            eval_inner(&expr, &fp)
        )
    }

    #[test]
    fn var_route_eval() {
        let fp = ffi::FlightPlan {
            route: "CINDY Z74 HAREM T104 ROKIL".to_string(),
            ..Default::default()
        };
        let expr = Expr::Ident("route".into());
        assert_eq!(
            Ok(Expr::Lit(Lit::Text("CINDY Z74 HAREM T104 ROKIL".into()))),
            eval_inner(&expr, &fp)
        )
    }
}

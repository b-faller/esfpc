#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;

use ast::eval;

#[cxx::bridge(namespace = "ffi")]
mod ffi {
    #[derive(Debug)]
    enum FpCheckResult {
        Ok,
        Route,
        RouteFlightLevel,
        EngineType,
        Navigation,
    }

    #[derive(Debug)]
    enum FlightRule {
        Vfr,
        Ifr,
        Yankee,
        Zulu,
    }

    #[derive(Debug)]
    struct FlightPlan {
        rule: FlightRule,
        rfl: i32,
    }

    extern "Rust" {
        fn check_flightplan(fp: FlightPlan) -> Result<FpCheckResult>;
    }
}

pub fn check_flightplan(fp: ffi::FlightPlan) -> Result<ffi::FpCheckResult, &'static str> {
    let rule = "true != ((\"test\" != \"notest\") == false)";
    let expr = parser::parse(rule).unwrap();

    eval(&expr).and(Ok(ffi::FpCheckResult::Ok))
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert_eq!(2 + 2, 4)
    }
}

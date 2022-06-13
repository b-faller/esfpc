#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;

use ast::eval;

#[cxx::bridge]
mod ffi {
    #[derive(Debug)]
    enum FpCheckResult {
        Ok,
        Route,
        RouteFlightLevel,
        EngineType,
        Navigation,
    }

    extern "Rust" {
        fn check_flightplan(rfl: i32) -> Result<FpCheckResult>;
    }
}

pub fn check_flightplan(rfl: i32) -> Result<ffi::FpCheckResult, &'static str> {
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

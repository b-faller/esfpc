#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;

use ast::eval;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn check_flightplan(rfl: i32) -> Result<()>;
    }
}

pub fn check_flightplan(rfl: i32) -> Result<(), &'static str> {
    let rule = "true != ((\"test\" != \"notest\") == false)";
    let expr = parser::parse(rule).unwrap();

    eval(&expr).and(Ok(()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert_eq!(2 + 2, 4)
    }
}

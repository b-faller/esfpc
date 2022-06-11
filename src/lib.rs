#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn check_flightplan(rfl: i32) -> Result<()>;
    }
}

pub fn check_flightplan(rfl: i32) -> Result<(), &'static str> {
    if rfl % 2000 == 0 {
        Ok(())
    } else {
        Err("RFL is odd")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert_eq!(2 + 2, 4)
    }
}

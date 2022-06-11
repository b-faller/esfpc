#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn add(a: i32, b: i32) -> i32;
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn test_func() -> i32 {
    666
}

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert_eq!(2 + 2, 4)
    }
}

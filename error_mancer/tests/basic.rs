#![feature(assert_matches)]

use std::assert_matches::assert_matches;
use std::num::{ParseIntError, TryFromIntError};

use error_mancer::prelude::*;

#[errors]
fn foo() -> Result<i32, _> {
    Ok(10)
}

#[test]
fn no_errors() {
    assert_eq!(foo().unwrap(), 10);
}

#[errors(ParseIntError, TryFromIntError)]
fn bar(x: &str) -> Result<u8, _> {
    std::fs::read("hello.txt")?;
    let result: i16 = x.parse()?;
    let result = result.try_into()?;
    Ok(result)
}

#[test]
fn specify_error_tests() {
    assert!(bar("10").is_ok());
    assert_matches!(bar("abc"), Err(BarError::ParseIntError(_)));
    assert_matches!(bar("300"), Err(BarError::TryFromIntError(_)));
}

mod module {
    use super::*;

    #[errors(TryFromIntError)]
    pub fn in_module() -> Result<(), _> {
        let _: u8 = 300_u16.try_into()?;
        Ok(())
    }
}

#[test]
fn pub_works() {
    assert_matches!(
        module::in_module(),
        Err(module::InModuleError::TryFromIntError(_))
    );
}

struct Test;

#[errors]
impl Test {
    #[errors(TryFromIntError)]
    fn method(&self, x: u16) -> Result<u8, _> {
        let x = x.try_into()?;
        Ok(x)
    }
}

use anyhow::Result;
use error_mancer::prelude::*;

#[errors]
fn no_errors_foo() -> Result<i32> {
    Ok(10)
}

#[test]
fn no_errors() {
    assert_eq!(no_errors_foo().unwrap(), 10);
}

#[errors]
fn double(x: i32) -> Result<i32> {
    Ok(x * 2)
}

#[test]
fn arguments() {
    assert_eq!(double(10).unwrap(), 20);
}

trait Foo {
    fn foo(&self) -> Result<i32>;
}

struct IShouldntError;

#[errors]
impl Foo for IShouldntError {
    #[errors]
    fn foo(&self) -> Result<i32> {
        Ok(10)
    }
}

#[test]
fn traits() {
    assert_eq!(IShouldntError.foo().unwrap(), 10);
}

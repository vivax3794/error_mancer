use error_mancer::errors;
use thiserror::Error;

#[derive(Error, Clone, Debug)]
#[error("err1")]
struct Err1;

#[errors(Err1)]
fn foo() -> Result<(), BarError> {
    Ok(())
}

#[errors(BarError)]
fn test() -> Result<(), _> {
    foo()?;
    Ok(())
}

#[errors(Err1)]
fn bar() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

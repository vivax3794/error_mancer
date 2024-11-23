use error_mancer::errors;
use thiserror::Error;

#[derive(Error, Clone, Debug)]
#[error("err1")]
struct Err1;

#[errors(Err1)]
#[derive(Clone)]
fn foo() -> Result<(), _> {
    Ok(())
}

fn bar() {
    let _ = foo().clone();
}

struct Bar;

#[errors]
impl Bar {
    #[errors(Err1)]
    #[derive(Clone)]
    fn bar(&self) -> Result<(), _> {
        Ok(())
    }
}

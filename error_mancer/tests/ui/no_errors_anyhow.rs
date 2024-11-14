use anyhow::Result;
use error_mancer::prelude::*;

#[errors]
fn foo() -> Result<i32> {
    let _ = std::fs::File::open("hello.txt")?;
    Ok(10)
}

fn main() {}

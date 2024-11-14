# error_mancer

## Overview

The `error_mancer` crate adds a `#[errors]` attribute that allows you to easily define and restrict error types in functions. This approach makes error handling more concise and keeps the error definitions close to the relevant methods, simplifying maintenance and modification.

## Example Usage

```rs
use std::io;

use error_mancer::*;

#[errors(io::Error, serde_json::Error)]
fn open_file() -> Result<SomeStruct, _> {
    let file = std::fs::File::open("hello.json")?;
    let data = serde_json::from_reader(file)?;

    Ok(data)
}

fn main() {
    match open_file() {
        Err(OpenFileError::IoError(err)) => { /* Handle I/O error */ },  
        Err(OpenFileError::SerdeJsonError(err)) => { /* Handle JSON parsing error */ },
        Ok(data) => { /* Use data */ }
    }
}
```

The main benefit of this approach is that it moves the error enum definition much closer to the method, making it easier to modify. Additionally, it supports generic error results like `anyhow`. In these cases, the return type is not modified, but the allowed return values are still restricted. This is particularly useful when implementing traits that require an `anyhow::Result`.

## Trait Implementation Example

```rs
use error_mancer::*;

#[errors]
impl other_crate::Trait for MyStruct {
    #[errors]
    fn some_method(&self) -> anyhow::Result<()> {
        // This would cause a compiler error now!
        // std::fs::open("hello.txt")?;
    }
}
```

## Design Goals

- **Simplified Error Wrapper Enums**: This crate aims to make defining trivial error wrapper enums much easier and more convenient.
- **Enforcing Error Restrictions**: It aims to allow you to enforce error restrictions on `anyhow::Result` and similar `Result` types.
- **Compatibility with `thiserror`**: This crate does **not** aim to replace `thiserror` or similar libraries. Instead, it encourages using them in tandem to define errors for use with `error_mancer`.

## Summary

`error_mancer` is a powerful yet simple tool for developers who want a more structured approach to error handling, particularly when working with complex traits or generic error types. By keeping error definitions close to their corresponding methods, it streamlines code management and enforces error consistency.

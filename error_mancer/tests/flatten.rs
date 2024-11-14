#![feature(assert_matches)]
#![no_std]

use core::assert_matches::assert_matches;

use error_mancer::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("error 1")]
struct Err1;

#[derive(Error, Debug)]
#[error("error 2")]
struct Err2;

#[derive(Error, Debug)]
#[error("error 3")]
struct Err3;

#[errors(Err1, Err2, Err3)]
fn foo(x: i32) -> Result<(), _> {
    match x {
        0 => Ok(()),
        1 => Err(Err1.into()),
        2 => Err(Err2.into()),
        _ => Err(Err3.into()),
    }
}

#[errors(FooError)]
fn wrapped(x: i32) -> Result<i32, _> {
    let result = foo(x);
    let result = match result {
        Err(FooError::Err3(_)) => 20,
        Ok(_) => 10,
        Err(err) => return Err(err.into()),
    };
    Ok(result)
}

#[errors(Err1, Err2, Err3)]
fn unwrapped(x: i32) -> Result<i32, _> {
    foo(x).into_super_error::<UnwrappedError>()?;
    Ok(10)
}

#[test]
fn test_unwrapped() {
    assert_matches!(unwrapped(0), Ok(10));
    assert_matches!(unwrapped(1), Err(UnwrappedError::Err1(Err1)));
    assert_matches!(unwrapped(2), Err(UnwrappedError::Err2(Err2)));
    assert_matches!(unwrapped(3), Err(UnwrappedError::Err3(Err3)));
}

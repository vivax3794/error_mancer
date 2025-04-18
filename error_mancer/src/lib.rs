//! The primary macro in this crate is `errors`, designed to simplify error handling by allowing developers to define and restrict error types directly within functions. This reduces boilerplate code and improves readability.
//!
//! # Usage
//!
//! ## Basic Example
//!
//! Below is a basic example of how to use the `errors` macro:
//!
//! ```rust
//! # use error_mancer::prelude::*;
//!
//! #[errors(std::io::Error)]
//! fn foo() -> Result<i32, _> {
//!     std::fs::File::open("hello.txt")?;
//!     Ok(10)
//! }
//!
//! fn bar() {
//!     match foo() {
//!         Err(FooError::StdIo(_)) => {/* Handle error */},
//!         Ok(_) => {/* Handle success */}
//!     }
//! }
//! ```
//!
//! This macro automatically generates an enum resembling the following and sets the `Result` error type to it, so developers do not need to manually define it:
//!
//! ```rust,ignore
//! // Auto generated code from `#[errors]`
//! #[derive(Debug)]
//! enum FooError {
//!     StdIo(std::io::Error),
//! }
//!
//! impl From<std::io::Error> for FooError { ... }
//! impl Display for FooError { ... }
//! impl Error for FooError {}
//! ```
//!
//! Defining no errors also works, which will generate an enum with no variants, enforcing that no errors are returned. This is useful for functions that are guaranteed not to fail but still require a `Result<...>` return type, such as in trait implementations. It provides extra safety by ensuring that no error paths are possible.
//!
//! ## Enum name
//! You can also explicitly set the enum name by using a Ident instead of `_` in the signature
//! ```rust
//! # use error_mancer::prelude::*;
//!
//! #[errors(std::io::Error)]
//! fn foo() -> Result<i32, MyErrorEnum> {
//!     std::fs::File::open("hello.txt")?;
//!     Ok(10)
//! }
//!
//! fn bar() {
//!     match foo() {
//!         Err(MyErrorEnum::StdIo(_)) => {/* ... */},
//!         Ok(_) => {/* ... */}
//!     }
//! }
//! ```
//!
//! ## Usage in `impl` Blocks
//!
//! To use the macro within an `impl` block, the block must also be annotated:
//!
//! ```rust
//! # use error_mancer::prelude::*;
//! # struct MyStruct;
//!
//! #[errors]
//! impl MyStruct {
//!     #[errors(std::io::Error)]
//!     fn method(&self) -> Result<(), _> {
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Usage with `anyhow::Result`
//!
//! The macro can also be used without overwriting an error type and is fully compatible with `anyhow::Result` and similar types. This is especially useful for developers who prefer using `anyhow` for general error handling but want to benefit from additional error type restrictions when needed, particularly in trait implementations:
//!
//! ```rust,compile_fail
//! # use error_mancer::prelude::*;
//!
//! #[errors]
//! fn foo() -> anyhow::Result<()> {
//!     // ❌ Compiler error: `std::fs::File::open(...)` returns `std::io::Error`,
//!     // but `#[errors]` enforces that no errors are allowed.
//!     std::fs::File::open("hello.txt")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Upcasting types
//! ```rust
//! # use error_mancer::prelude::*;
//! # use thiserror::Error;
//! # #[derive(Error, Debug)]
//! # #[error("1")]
//! # struct Err1;
//! # #[derive(Error, Debug)]
//! # #[error("2")]
//! # struct Err2;
//! # #[derive(Error, Debug)]
//! # #[error("3")]
//! # struct Err3;
//!
//! #[errors(Err1, Err2)]
//! fn foo() -> Result<i32, _> {
//!     // ...
//!     # todo!()
//! }
//!
//! #[errors(Err1, Err2, Err3)]
//! fn bar() -> Result<i32, _> {
//!     let result = foo().into_super_error::<BarError>()?;
//!     Ok(result)
//! }
//! ```
//!
//! ## Deriving traits for generated enum
//! You can annotate the function with `#[derive]` to derive traits for the generated enum.
//! Note that the `#[derive]` macro must be used after the `errors` macro. (technically in `impl`
//! blocks the order doesnt matter, but we recommend using `#[derive]` after `errors` for consistency.)
//! ```rust
//! # use error_mancer::prelude::*;
//! # use thiserror::Error;
//! # #[derive(Error, Debug, Clone)]
//! # #[error("1")]
//! # struct Err1;
//!
//! #[errors(Err1)]
//! #[derive(Clone)]
//! fn foo() -> Result<(), _> {
//!     Ok(())
//! }
//!
//! fn bar() {
//!     let _ = foo().clone();
//! }
//! ```
//!
//! # Specifics and Implementation Details
//!
//! ## Error Type Overwriting
//!
//! The macro looks for a type named `Result` in the root of the return type. If the second generic argument is `_`, it replaces it with the appropriate error type. See the examples below:
//!
//! | Original                    | Modified                                         |
//! | --------------------------- | ------------------------------------------------ |
//! | `Result<T, _>`              | `Result<T, FooError>`                            |
//! | `Result<T, CustomName>`     | `Result<T, CustomName>`                          |
//! | `Result<T, Box<dyn Error>>` | `Result<T, Box<dyn Error>>`                      |
//! | `std::result::Result<T, _>` | `std::result::Result<T, FooError>`               |
//! | `anyhow::Result<T>`         | `anyhow::Result<T>`                              |
//! | `Vec<Result<T, _>>`         | ❌ compiler error, nested types arent replaced   |
//!
//! ## Enum Visibility
//!
//! The enum can either be emitted inside the function body, or alongside the function (in which
//! case it takes on the functions visibility) based on the
//! following conditions:
//! * If the second generic argument in `Result` is `_` its emitted outside the function.
//! * If the second generic argument in `Result` is a simple identifier its emitted outside.
//! * Otherwise, the enum is generated inside the function body and cannot be accessed externally.
//!
//! (in the following example the enums are include to illustrate where the macro would
//! generate them.)
//! ```rust,ignore
//! #[errors]
//! fn foo() -> Result<(), _> { ... }
//!
//! // enum FooError {...}
//!
//! #[errors]
//! pub fn bar() -> Result<(), _> { ... }
//!
//! // pub enum FooError {...}
//!
//! #[errors]
//! pub fn baz() -> Result<(), CustomName> { ... }
//!
//! // pub enum CustomName {...}
//!
//! #[errors]
//! pub fn secret() -> anyhow::Result<()> {
//!     // enum SecretError { ... }
//!     // ...
//! }
//! ```
//!
//! ## Naming Conventions
//!
//! The enum name is derived from the function name, converted to Pascal case using the `case_fold` crate to conform to Rust naming conventions for types and enums. Similarly, variant names are derived from the path segments of the types, with the "Error" suffix removed if present. For example, `std::io::Error` would produce a variant called `StdIo`, while `io::Error` would produce `Io`.
//!
//! ## Display Implementation
//!
//! The `Display` implementation simply delegates to each contained error, ensuring consistent and readable error messages.
//!
//! ## `into_super_error`
//! This function uses the `FlattenInto` trait which is automatically implemented by the macro for
//! its errors, for all target types which implemnt `From<...>` for each of the errors variants. i.e a generated
//! implementation might look like:
//! ```rust
//! # use error_mancer::{FlattenInto, errors};
//! # use thiserror::Error;
//! # #[derive(Error, Debug)]
//! # #[error("1")]
//! # struct Err1;
//! # #[derive(Error, Debug)]
//! # #[error("2")]
//! # struct Err2;
//! # enum OurError {
//! #   Err1(Err1),
//! #   Err2(Err2)
//! # }
//!
//! impl<T> FlattenInto<T> for OurError
//!     where T: From<Err1> + From<Err2> {
//!         fn flatten(self) -> T {
//!             match self {
//!                 Self::Err1(err) => T::from(err),
//!                 Self::Err2(err) => T::from(err),
//!             }
//!         }
//!     }
//! ```
#![no_std]

pub use error_mancer_macros::errors;

pub mod prelude {
    pub use error_mancer_macros::errors;

    pub use super::ResultExt;
}

#[doc(hidden)]
#[diagnostic::on_unimplemented(
    message = "Error `{T}` not allowed to be returned from this function.",
    label = "`{T}` is not listed in `#[errors]` attribute",
    note = "Add `{T}` to `#[errors]` attribute or handle this error locally."
)]
pub trait ErrorMancerFrom<T> {
    fn from(value: T) -> Self;
}

/// This trait allows a error to be flattened into another one and is automatically implemented by
/// the `#[errors]` macro for all super errors that implement `From<...>` for each of its fields.
pub trait FlattenInto<T> {
    fn flatten(self) -> T;
}

/// This trait extends `Result` with an additional method to upcast a error enum.
pub trait ResultExt<T, E> {
    /// This will convert from the current `E` into the specified super error.
    fn into_super_error<S>(self) -> Result<T, S>
    where
        E: FlattenInto<S>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    #[inline(always)]
    fn into_super_error<S>(self) -> Result<T, S>
    where
        E: FlattenInto<S>,
    {
        self.map_err(|err| err.flatten())
    }
}

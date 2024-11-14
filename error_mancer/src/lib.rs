//! # `errors` Macro Documentation
//!
//! ## Overview
//!
//! The primary macro in this crate is `errors`, designed to simplify error handling by allowing developers to define and restrict error types directly within functions. This reduces boilerplate code and improves readability.
//!
//! ## Usage
//!
//! ### Basic Example
//!
//! Below is a basic example of how to use the `errors` macro:
//!
//! ```rust
//! #[errors(std::io::Error)]
//! fn foo() -> Result<i32, _> {
//!     std::fs::open("hello.txt")?;
//!     Ok(10)
//! }
//! ```
//!
//! This macro automatically generates an enum resembling the following and sets the `Result` error type to it, so developers do not need to manually define it:
//!
//! ```rust
//! #[derive(Debug)]
//! enum FooError {
//!     StdIoError(std::io::Error),
//! }
//!
//! impl From<std::io::Error> for FooError { ... }
//! impl Display for FooError { ... }
//! impl Error for FooError {}
//! ```
//!
//! Defining no errors also works, which will generate an enum with no variants, enforcing that no errors are returned. This is useful for functions that are guaranteed not to fail but still require a `Result<...>` return type, such as in trait implementations. It provides extra safety by ensuring that no error paths are possible.
//!
//! ### Usage in `impl` Blocks
//!
//! To use the macro within an `impl` block, the block must also be annotated:
//!
//! ```rust
//! #[errors]
//! impl MyStruct {
//!     #[errors(std::io::Error)]
//!     fn method(&self) -> Result<(), _> {
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ### Usage with `anyhow::Result`
//!
//! The macro can also be used without overwriting an error type and is fully compatible with `anyhow::Result` and similar types. This is especially useful for developers who prefer using `anyhow` for general error handling but want to benefit from additional error type restrictions when needed, particularly in trait implementations:
//!
//! ```rust
//! #[errors]
//! fn foo() -> anyhow::Result<()> {
//!     // This would cause a compiler error
//!     // std::fs::open("hello.txt")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Specifics and Implementation Details
//!
//! ### Error Type Overwriting
//!
//! The macro looks for a type named `Result` in the root of the return type. If the second generic argument is `_`, it replaces it with the appropriate error type. See the examples below:
//!
//! | Original                    | Modified                                         |
//! | --------------------------- | ------------------------------------------------ |
//! | `Result<T, _>`              | `Result<T, FooError>`                            |
//! | `std::result::Result<T, _>` | `std::result::Result<T, FooError>`               |
//! | `anyhow::Result<T>`         | `anyhow::Result<T>`                              |
//! | `Vec<Result<T, _>>`         | `Vec<Result<T, _>>`, leading to a compiler error |
//!
//! ### Enum Visibility
//!
//! The generated enum takes on the visibility of the function. The only exception is when the error type is not replaced, such as with `anyhow::Result`. In this case, the enum is emitted inside the function body, making it inaccessible to the rest of the module.
//!
//! ### Naming Conventions
//!
//! The enum name is derived from the function name, converted to Pascal case using the `case_fold` crate to conform to Rust naming conventions for types and enums. Similarly, variant names are derived from the path segments of the types. For example, `std::io::Error` would produce a variant called `StdIoError`, while `io::Error` would produce `IoError`.
//!
//! ### Display Implementation
//!
//! The `Display` implementation simply delegates to each contained error, ensuring consistent and readable error messages.

pub use error_mancer_macros::errors;

pub mod prelude {
    pub use error_mancer_macros::errors;

    pub use super::handle;
}

#[macro_export]
macro_rules! handle {
    {$expr:expr => $error:path {propagate ($($return_error:ident),*)$(,$pat:pat => $res:expr)*}} => {
        {
            use $error::*;
            match $expr {
                $(std::result::Result::Err($return_error(err)) => return Err(err.into()),)*
                $($pat => $res,)*
            }
        }
    };
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

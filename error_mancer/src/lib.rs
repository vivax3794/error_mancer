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

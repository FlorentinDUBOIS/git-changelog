//! # Library module
//!
//! The library module expose macro which help during development

#[macro_export]
macro_rules! ok {
    () => {
        Ok(())
    };
    ($x:expr) => {
        Ok($x)
    };
}

#[macro_export]
macro_rules! err {
    ($( $x:expr ),*) => {
        Err(failure::format_err!($( $x, )*))
    };
}

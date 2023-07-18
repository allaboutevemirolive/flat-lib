// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

use std::error::Error as StdError;
use std::fmt::Display;

pub struct Error(/* ... */);

impl Error {
    pub(crate) fn from_fmt<T: Display>(_error: T) -> Self {
        println!("called Error::from_fmt");
        Error {}
    }

    pub(crate) fn from_std_error<T: StdError>(error: T) -> Self {
        _ = error.source();
        println!("called Error::from_std_error");
        Error {}
    }
}

macro_rules! anyhow {
    ($err:expr) => ({
        // The macro starts by capturing the expression provided as $err.
        // The expression can be any valid Rust expression.
        // $err:expr is a metavariable that represents the captured expression.
        
        // The #[allow(unused_imports)] attribute is used to suppress the unused_imports warning.
        // TODO: unused import?
        #[allow(unused_imports)]
        use $crate::{DisplayKind, StdErrorKind};
        
        // The match expression starts.
        match $err {
            // Here, the macro expands into a match arm that matches the provided expression.
            // It captures the expression as `error`.
            error => (&error).anyhow_kind().new(error),
        }
    });
}


struct DisplayTag;

trait DisplayKind {
    #[inline]
    fn anyhow_kind(&self) -> DisplayTag {
        DisplayTag
    }
}

impl<T: Display> DisplayKind for &T {}

impl DisplayTag {
    #[inline]
    fn new<M: Display>(self, message: M) -> Error {
        Error::from_fmt(message)
    }
}

// =====================

struct StdErrorTag;

trait StdErrorKind {
    #[inline]
    fn anyhow_kind(&self) -> StdErrorTag {
        StdErrorTag
    }
}

impl<T: StdError> StdErrorKind for T {}

impl StdErrorTag {
    #[inline]
    fn new<E: StdError>(self, error: E) -> Error {
        Error::from_std_error(error)
    }
}

fn main() {
    let _err = anyhow!("oh no!");

    let io_error = std::fs::read("/tmp/nonexist").unwrap_err();
    let _err = anyhow!(io_error);
}
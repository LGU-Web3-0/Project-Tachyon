pub use anyhow::Result;
use std::process::abort;

pub trait LoggedUnwrap {
    type Output;
    fn logged_unwrap(self) -> Self::Output;
}

pub trait IntoAnyhow {
    type Output;
    fn anyhow(self) -> Result<Self::Output>;
}

impl<T> LoggedUnwrap for Result<T> {
    type Output = T;

    fn logged_unwrap(self) -> Self::Output {
        match self {
            Ok(x) => x,
            Err(y) => {
                let bt = std::backtrace::Backtrace::force_capture();
                log::error!("{}\n{}", y, bt);
                abort()
            }
        }
    }
}

impl<T, E: Into<anyhow::Error>> IntoAnyhow for core::result::Result<T, E> {
    type Output = T;

    fn anyhow(self) -> Result<Self::Output> {
        self.map_err(Into::into)
    }
}

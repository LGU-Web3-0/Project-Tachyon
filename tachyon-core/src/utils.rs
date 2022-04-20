pub use anyhow::Result;

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
                log::error!("{}", y);
                panic!("server panics due to above error: {}", y);
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

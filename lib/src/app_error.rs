use crate::core::WORDS_FILE_PATH;
use error_stack::Context;
use kinded::Kinded;
use sealed::sealed;
use std::fmt::{
    self, Display, Formatter,
};

pub type AppResult<T> =
    error_stack::Result<T, AppError>;

#[sealed]
pub trait AppResultExt<T> {
    fn err_as_string(
        self,
    ) -> Result<T, String>;
}
#[sealed]
impl<T> AppResultExt<T>
    for AppResult<T>
{
    fn err_as_string(
        self,
    ) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

#[derive(Debug, Kinded)]
pub enum AppError {
    NoWords,
    StdIoRead,
    InvalidCharset,
    InvalidGuessLength(usize),
}

impl Display for AppError {
    fn fmt(
        &self,
        f: &mut Formatter,
    ) -> fmt::Result {
        use AppError as E;

        match self {
            E::NoWords => {
                write!(f, "[{:?}] No words found in file.", E::NoWords)
            }
            E::StdIoRead => {
                write!(f, "[{:?}] Failed to read stdio.", E::StdIoRead)
            }
            E::InvalidCharset => {
                write!(
                    f,
                    "[{:?}] The Words file ('{}') contains invalid UTF-8 characters",
                    E::InvalidCharset,
                    WORDS_FILE_PATH,
                )
            }
            E::InvalidGuessLength(
                expected_len,
            ) => {
                write!(
                    f,
                    "[{:?}] Your guess word must be {} letters long.",
                    AppErrorKind::InvalidGuessLength,
                    expected_len
                )
            }
        }
    }
}
impl Context for AppError {}

#[cfg(test)]
mod tests {

    #[macro_export]
    macro_rules! assert_app_error {
        ($actual:ident, $expected:ident) => {
            pretty_assertions::assert_eq!(
                $actual
                    .unwrap_err()
                    .to_string(),
                $expected.to_string()
            );
        };
    }
}

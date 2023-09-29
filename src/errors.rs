use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] ErrorRepr);

#[derive(Error, Debug)]
pub(crate) enum ErrorRepr {
    #[error("integer to generic conversion error")]
    IntToGenError,
    #[error("integer to usize conversion error")]
    IntToUsizeError,
}

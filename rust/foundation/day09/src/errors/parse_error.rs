use thiserror::Error;
use crate::errors::transfer_error::TransferError;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {

    #[error("invalid amount format")]
    InvalidAmountFormat,


    #[error("invalid index format")]
    InvalidIndexFormat,


    #[error("invalid mint format")]
    InvalidMintFormat,

    #[error("invalid name format")]
    InvalidNameFormat,
}

impl From<ParseError> for TransferError {
    fn from(e: ParseError) -> Self {
        TransferError::InvalidInput(e.to_string())
    }
}

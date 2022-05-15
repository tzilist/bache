use bytes::Bytes;
use thiserror::Error;
use tokio::task::JoinError;
use tonic::{Code, Status};

use crate::domain::{DigestHash, InstanceName};

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    InvalidHexString(#[from] hex::FromHexError),

    #[error(transparent)]
    Tokio(#[from] JoinError),

    #[error("Store for `instance_name` of {0} was not found")]
    StoreNotFound(InstanceName),

    #[error("Digest with hash {0} was not found")]
    DigestInfoNotFound(DigestHash),

    #[error("Invalid digest part(s), {0}")]
    InvalidDigestParts(String),

    #[error("`{0}` is not a valid resource name")]
    InvalidResourceName(String),

    #[error("`{0}` could not be converted to a different int type")]
    ConversionIntError(String),
}

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidHexString(from_hex_err) => match from_hex_err {
                hex::FromHexError::InvalidHexCharacter { c, index } => Status::with_details(
                    Code::InvalidArgument,
                    format!("Invalid hex character {c} found at index {index}"),
                    Bytes::from_static(
                        "Valid hex characters are . Valid characters are `0...9`, `a...f`, or \
                         `A...F`"
                            .as_bytes(),
                    ),
                ),
                hex::FromHexError::OddLength => Status::with_details(
                    Code::InvalidArgument,
                    "Invalid hex had an odd length",
                    Bytes::from_static(
                        "A hex string's length needs to be even, as two digits correspond to one \
                         bytes"
                            .as_bytes(),
                    ),
                ),
                hex::FromHexError::InvalidStringLength => Status::with_details(
                    Code::Internal,
                    "Hex container was of invalid length",
                    Bytes::from_static(
                        "the hex string's length * 2 has to match the container's length"
                            .as_bytes(),
                    ),
                ),
            },
            Error::Tokio(join_error) => Status::with_details(
                Code::Internal,
                "Tokio task failed to execute",
                Bytes::from(join_error.to_string()),
            ),
            err @ Error::StoreNotFound(_) => Status::internal(err.to_string()),
            err @ Error::InvalidResourceName(_) => Status::invalid_argument(err.to_string()),
            err @ Error::DigestInfoNotFound(_) => Status::not_found(err.to_string()),
            err @ Error::ConversionIntError(_) => Status::invalid_argument(err.to_string()),
            err @ Error::InvalidDigestParts(_) => Status::invalid_argument(err.to_string()),
        }
    }
}

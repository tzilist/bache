use bytes::Bytes;
use thiserror::Error;
use tokio::task::JoinError;
use tonic::{Code, Status};

use crate::domain::InstanceName;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    InvalidHexString(#[from] hex::FromHexError),

    #[error(transparent)]
    Tokio(#[from] JoinError),

    #[error("Store {0} was not found")]
    StoreNotFound(InstanceName),

    #[error("{0} is not a valid resource name")]
    InvalidResourceName(String),
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
            Error::StoreNotFound(instance_name) => {
                Status::internal(format!("Store {instance_name} was not found"))
            }
            Error::InvalidResourceName(resource_name) => {
                Status::invalid_argument(format!("{resource_name} is not a valid resource name"))
            }
        }
    }
}

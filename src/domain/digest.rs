use std::hash::Hash;

use hex::FromHex;

use crate::{errors::Error, protos::build::bazel::remote::execution::v2::Digest};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DigestInfo {
    pub size_bytes: i64,
    pub packed_hash: [u8; 32],
}

impl DigestInfo {
    pub fn new(packed_hash: [u8; 32], size_bytes: i64) -> Self {
        Self {
            size_bytes,
            packed_hash,
        }
    }
}

impl TryFrom<Digest> for DigestInfo {
    type Error = Error;
    fn try_from(digest: Digest) -> Result<Self, Self::Error> {
        let packed_hash = <[u8; 32]>::from_hex(digest.hash).map_err(Error::from)?;

        Ok(Self {
            size_bytes: digest.size_bytes,
            packed_hash,
        })
    }
}

impl From<DigestInfo> for Digest {
    fn from(digest_info: DigestInfo) -> Self {
        Self {
            hash: hex::encode(digest_info.packed_hash),
            size_bytes: digest_info.size_bytes,
        }
    }
}

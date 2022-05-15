use std::{fmt, hash::Hash};

use hex::{FromHex, ToHex};

use crate::{errors::Error, protos::build::bazel::remote::execution::v2::Digest};

/// Wrapped type of a hash sent by bazel
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DigestHash(String);

impl fmt::Display for DigestHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Contains the digest info on a chunk of data
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

    pub fn try_new<T>(hash: &str, size_bytes: T) -> Result<Self, Error>
    where
        T: TryInto<i64>,
    {
        let packed_hash = <[u8; 32]>::from_hex(hash)
            .map_err(|_| Error::InvalidDigestParts("unable to create `packed_hash`".to_string()))?;
        let size_bytes = size_bytes.try_into().map_err(|_| {
            Error::InvalidDigestParts("could not convert `size_bytes` to i64".to_string())
        })?;

        Ok(DigestInfo {
            packed_hash,
            size_bytes,
        })
    }

    pub fn hash(&self) -> DigestHash {
        DigestHash(self.packed_hash.encode_hex::<String>())
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

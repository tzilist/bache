use bytes::Bytes;
use moka::future::Cache;

use crate::models::digest::DigestInfo;

pub struct MemoryStore {
    cache: Cache<DigestInfo, Bytes>,
}

use std::fmt::Debug;

use async_trait::async_trait;
use bytes::Bytes;
use moka::future::Cache;
use tracing::instrument;

use super::Store;
use crate::{domain::DigestInfo, errors::Error};

#[derive(Clone)]
pub struct MemoryStore {
    cache: Cache<DigestInfo, Bytes>,
}

impl Debug for MemoryStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryStore").finish_non_exhaustive()
    }
}

#[async_trait]
impl Store for MemoryStore {
    #[instrument(skip(self))]
    async fn contains_key(&self, key: &DigestInfo) -> bool {
        self.cache.contains_key(key)
    }

    #[instrument(skip(self))]
    async fn get_chunk(
        &self,
        key: &DigestInfo,
        offset: usize,
        limit: usize,
    ) -> Result<Bytes, Error> {
        todo!()
    }
}

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bytes::Bytes;
use enum_dispatch::enum_dispatch;

use self::memory::MemoryStore;
use crate::{
    domain::{DigestInfo, InstanceName},
    errors::Error,
};

pub mod memory;

#[async_trait]
#[enum_dispatch]
pub trait Store {
    async fn contains_key(&self, key: &DigestInfo) -> bool;

    async fn get_chunk(
        &self,
        key: &DigestInfo,
        offset: usize,
        limit: usize,
    ) -> Result<Bytes, Error>;
}

#[enum_dispatch(Store)]
#[derive(Clone, Debug)]
pub enum StoreKind {
    Memory(MemoryStore),
}

pub struct StoreManager {
    stores: HashMap<InstanceName, Arc<StoreKind>>,
}

impl StoreManager {
    pub fn new(stores: HashMap<InstanceName, Arc<StoreKind>>) -> Self {
        Self { stores }
    }

    pub fn get_store_by_instance_name(
        &self,
        instance_name: &InstanceName,
    ) -> Result<Arc<StoreKind>, Error> {
        let store = self
            .stores
            .get(instance_name)
            .ok_or_else(|| Error::StoreNotFound(instance_name.clone()))?
            .to_owned();

        Ok(store)
    }
}

use std::{collections::HashMap, pin::Pin};

use futures::stream::Stream;
use tonic::{async_trait, Request, Response, Status};
use tracing::instrument;

use crate::protos::build::bazel::remote::execution::v2::{
    content_addressable_storage_server::ContentAddressableStorage, BatchReadBlobsRequest,
    BatchReadBlobsResponse, BatchUpdateBlobsRequest, BatchUpdateBlobsResponse,
    FindMissingBlobsRequest, FindMissingBlobsResponse, GetTreeRequest, GetTreeResponse,
};

pub struct ContentAddressableStorageService {
    stores: HashMap<(), ()>,
}

impl ContentAddressableStorageService {
    pub fn new(stores: HashMap<(), ()>) -> Self {
        Self { stores }
    }
}

#[async_trait]
impl ContentAddressableStorage for ContentAddressableStorageService {
    #[instrument(err, skip(self))]
    async fn find_missing_blobs(
        &self,
        _request: Request<FindMissingBlobsRequest>,
    ) -> Result<Response<FindMissingBlobsResponse>, Status> {
        Ok(Response::new(FindMissingBlobsResponse::default()))
    }

    #[instrument(err, skip(self))]
    async fn batch_update_blobs(
        &self,
        _request: Request<BatchUpdateBlobsRequest>,
    ) -> Result<Response<BatchUpdateBlobsResponse>, Status> {
        Ok(Response::new(BatchUpdateBlobsResponse::default()))
    }

    #[instrument(err, skip(self))]
    async fn batch_read_blobs(
        &self,
        _request: Request<BatchReadBlobsRequest>,
    ) -> Result<Response<BatchReadBlobsResponse>, Status> {
        Ok(Response::new(BatchReadBlobsResponse::default()))
    }

    type GetTreeStream = Pin<Box<dyn Stream<Item = Result<GetTreeResponse, Status>> + Send>>;

    #[instrument(err, skip(self))]
    async fn get_tree(
        &self,
        _request: Request<GetTreeRequest>,
    ) -> Result<Response<Self::GetTreeStream>, Status> {
        todo!();
    }
}

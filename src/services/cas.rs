use std::pin::Pin;

use async_trait::async_trait;
use futures::stream::{BoxStream, FuturesUnordered, Stream};
use tonic::{Request, Response, Status};
use tracing::instrument;

use crate::{
    domain::{DigestInfo, InstanceName},
    errors::Error,
    infrastructure::{Store, StoreManager},
    protos::build::bazel::remote::execution::v2::{
        content_addressable_storage_server::{
            ContentAddressableStorage, ContentAddressableStorageServer,
        },
        BatchReadBlobsRequest, BatchReadBlobsResponse, BatchUpdateBlobsRequest,
        BatchUpdateBlobsResponse, Digest, FindMissingBlobsRequest, FindMissingBlobsResponse,
        GetTreeRequest, GetTreeResponse,
    },
};

pub struct ContentAddressableStorageService {
    stores: StoreManager,
}

impl ContentAddressableStorageService {
    pub fn new(stores: StoreManager) -> Self {
        Self { stores }
    }

    pub fn into_server(self) -> ContentAddressableStorageServer<ContentAddressableStorageService> {
        ContentAddressableStorageServer::new(self)
    }
}

#[async_trait]
impl ContentAddressableStorage for ContentAddressableStorageService {
    #[instrument(err, skip(self))]
    async fn find_missing_blobs(
        &self,
        request: Request<FindMissingBlobsRequest>,
    ) -> Result<Response<FindMissingBlobsResponse>, Status> {
        let FindMissingBlobsRequest {
            instance_name,
            blob_digests,
        } = request.into_inner();

        let instance_name = InstanceName::new(instance_name);
        let store = self.stores.get_store_by_instance_name(&instance_name)?;

        let join_handles = FuturesUnordered::new();
        for digest in blob_digests {
            let digest: DigestInfo = digest.try_into()?;
            let store = store.clone();

            join_handles.push(tokio::spawn(async move {
                if store.contains_key(&digest).await {
                    None
                } else {
                    Some(digest)
                }
            }));
        }

        let missing_blob_digests = futures::future::try_join_all(join_handles)
            .await
            .map_err(Error::from)?
            .into_iter()
            .filter_map(|digest_info| digest_info.map(Digest::from))
            .collect();

        Ok(Response::new(FindMissingBlobsResponse {
            missing_blob_digests,
        }))
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
        request: Request<BatchReadBlobsRequest>,
    ) -> Result<Response<BatchReadBlobsResponse>, Status> {
        let BatchReadBlobsRequest {
            instance_name,
            digests,
            acceptable_compressors: _,
        } = request.into_inner();

        let instance_name = InstanceName::new(instance_name);
        let store = self.stores.get_store_by_instance_name(&instance_name)?;

        // let join_handles = FuturesUnordered::new();

        for digest in digests {
            let digest: DigestInfo = digest.try_into()?;
            let store = store.clone();

            // join_handles.push(tokio::spawn(async move {
            //     let size_bytes: usize = digest.size_bytes.try_into().map_err(|_| {
            //         Status::invalid_argument(
            //             "digest's `size_bytes` could not be converted into a valid usize",
            //         )
            //     })?;

            //     store
            //         .get_part_unchunked(digest, 0, None, Some(size_bytes))
            //         .await
            // }))
        }

        // futures::future::try_join_all(join_handles);

        Ok(Response::new(BatchReadBlobsResponse::default()))
    }

    type GetTreeStream = BoxStream<'static, Result<GetTreeResponse, Status>>;

    #[instrument(err, skip(self))]
    async fn get_tree(
        &self,
        _request: Request<GetTreeRequest>,
    ) -> Result<Response<Self::GetTreeStream>, Status> {
        todo!();
    }
}

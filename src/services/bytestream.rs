use async_trait::async_trait;
use futures::stream::BoxStream;
use tonic::{Request, Response, Status, Streaming};

use crate::{
    domain::{DigestInfo, ResourceName},
    infrastructure::{Store, StoreManager},
    protos::google::bytestream::{
        byte_stream_server::{ByteStream, ByteStreamServer},
        QueryWriteStatusRequest, QueryWriteStatusResponse, ReadRequest, ReadResponse, WriteRequest,
        WriteResponse,
    },
};

pub struct ByteStreamService {
    stores: StoreManager,
}

impl ByteStreamService {
    pub fn new(stores: StoreManager) -> Self {
        Self { stores }
    }

    pub fn into_server(self) -> ByteStreamServer<Self> {
        ByteStreamServer::new(self)
    }

    /// small utility function to create a stream and send a read response
    fn stream_one_read_response(
        &self,
        read_response: ReadResponse,
    ) -> Result<Response<BoxStream<'static, Result<ReadResponse, Status>>>, Status> {
        Ok(Response::new(Box::pin(tokio_stream::once(Ok(
            read_response,
        )))))
    }
}

#[async_trait]
impl ByteStream for ByteStreamService {
    type ReadStream = BoxStream<'static, Result<ReadResponse, Status>>;

    async fn read(
        &self,
        request: Request<ReadRequest>,
    ) -> Result<Response<Self::ReadStream>, Status> {
        let ReadRequest {
            resource_name,
            read_offset,
            read_limit,
        } = request.into_inner();

        let read_limit: usize = read_limit.try_into().map_err(|_| {
            Status::invalid_argument("`read_limit` could not be converted into a valid usize")
        })?;

        if read_limit == 0 {
            return self.stream_one_read_response(ReadResponse::default());
        }

        let read_offset: usize = read_offset.try_into().map_err(|_| {
            Status::invalid_argument("`read_limit` could not be converted into a valid usize")
        })?;

        let resource_name = ResourceName::try_from(resource_name)?;
        let digest_info = DigestInfo::try_new(&resource_name.hash, resource_name.size)?;

        let store = self
            .stores
            .get_store_by_instance_name(&resource_name.instance_name)?;

        let bytes_chunk = store
            .get_chunk(&digest_info, read_offset, read_limit)
            .await?;

        if bytes_chunk.is_empty() {
            self.stream_one_read_response(ReadResponse::default())
        } else {
            self.stream_one_read_response(ReadResponse {
                data: bytes_chunk.to_vec(),
            })
        }
    }

    async fn write(
        &self,
        _request: Request<Streaming<WriteRequest>>,
    ) -> Result<Response<WriteResponse>, Status> {
        todo!()
    }

    async fn query_write_status(
        &self,
        _request: Request<QueryWriteStatusRequest>,
    ) -> Result<Response<QueryWriteStatusResponse>, Status> {
        todo!()
    }
}

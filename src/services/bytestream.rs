use async_trait::async_trait;
use futures::{
    stream::{self, BoxStream},
    Stream,
};
use tonic::{Request, Response, Status, Streaming};

use crate::{
    domain::ResourceName,
    infrastructure::StoreManager,
    protos::google::bytestream::{
        byte_stream_server::{ByteStream, ByteStreamServer},
        QueryWriteStatusRequest, QueryWriteStatusResponse, ReadRequest, ReadResponse, WriteRequest,
        WriteResponse,
    },
};

pub struct ByteStreamService {
    stores: StoreManager,
    max_bytes_per_stream: usize,
}

impl ByteStreamService {
    pub fn new(stores: StoreManager, max_bytes_per_stream: usize) -> Self {
        Self {
            stores,
            max_bytes_per_stream,
        }
    }

    pub fn into_server(self) -> ByteStreamServer<Self> {
        ByteStreamServer::new(self)
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

        let read_limit: usize = read_limit.try_into().map_err(|e| {
            Status::invalid_argument("`read_limit` could not be converted into a valid usize")
        })?;

        if read_limit == 0 {
            return Ok(Response::new(Box::pin(stream::once(async {
                Ok(ReadResponse { data: Vec::new() })
            }))));
        }

        let read_offset: usize = read_offset.try_into().map_err(|e| {
            Status::invalid_argument("`read_limit` could not be converted into a valid usize")
        })?;

        let resource_name = ResourceName::try_from(resource_name)?;

        let store = self
            .stores
            .get_store_by_instance_name(&resource_name.instance_name);

        todo!()
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

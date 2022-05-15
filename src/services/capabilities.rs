use async_trait::async_trait;
use tonic::{Request, Response, Status};
use tracing::instrument;

use crate::protos::build::bazel::{
    remote::execution::v2::{
        capabilities_server::{Capabilities, CapabilitiesServer},
        compressor::Value as Compressor,
        digest_function::Value as DigestFunction,
        symlink_absolute_path_strategy::Value as SymlinkAbsolutePathStrategy,
        ActionCacheUpdateCapabilities, CacheCapabilities, GetCapabilitiesRequest,
        ServerCapabilities,
    },
    semver::SemVer,
};

pub struct CapabilitiesService;

impl CapabilitiesService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_server(self) -> CapabilitiesServer<CapabilitiesService> {
        CapabilitiesServer::new(self)
    }
}

#[async_trait]
impl Capabilities for CapabilitiesService {
    #[instrument(err, skip(self))]
    async fn get_capabilities(
        &self,
        _request: Request<GetCapabilitiesRequest>,
    ) -> Result<Response<ServerCapabilities>, Status> {
        Ok(Response::new(ServerCapabilities {
            cache_capabilities: Some(CacheCapabilities {
                digest_functions: vec![DigestFunction::Sha256.into()],
                action_cache_update_capabilities: Some(ActionCacheUpdateCapabilities {
                    update_enabled: true,
                }),
                cache_priority_capabilities: None,
                max_batch_total_size_bytes: 128 * 1024, // max 128MB batch sizes
                symlink_absolute_path_strategy: SymlinkAbsolutePathStrategy::Disallowed.into(),
                supported_compressors: vec![Compressor::Identity.into()],
                supported_batch_update_compressors: vec![Compressor::Identity.into()],
            }),
            execution_capabilities: None,
            deprecated_api_version: None,
            low_api_version: Some(SemVer {
                major: 2,
                minor: 0,
                patch: 0,
                prerelease: String::new(),
            }),
            high_api_version: Some(SemVer {
                major: 2,
                minor: 0,
                patch: 0,
                prerelease: String::new(),
            }),
        }))
    }
}

use clap::Parser;

use crate::tracing::TracingConfig;

#[derive(Parser, Debug, Clone)]
#[clap(rename_all = "kebab-case", next_help_heading = "SERVER CONFIGS")]
pub struct ServerConfig {
    /// Port to listen to gRPC requests
    #[clap(long, env = "BACHE_GRPC_PORT", default_value_t = 50051)]
    pub grpc_port: u32,

    /// Host name for gRPC requests
    #[clap(long, env = "BACHE_GRPC_HOSTNAME", default_value = "0.0.0.0")]
    pub grpc_hostname: String,

    /// Disable the gRPC reflection API
    #[clap(long, env = "BACHE_DISABLE_GRPC_REFELCTION")]
    pub disable_grpc_reflection: bool,

    /// Disable health checks. Used only for testing
    #[clap(long, env = "BACHE_DISABLE_HEALTH_CHECKS")]
    pub disable_health_checks: bool,
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(flatten)]
    pub server_config: ServerConfig,

    #[clap(flatten)]
    pub tracing_config: TracingConfig,
}

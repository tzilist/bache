use std::net::SocketAddr;

use eyre::WrapErr;
use tonic::transport::Server;

use crate::config::Args;

fn create_socket_address(hostname: &str, port: u32) -> eyre::Result<SocketAddr> {
    format!("{hostname}:{port}").parse().wrap_err(
        "Failed to create socket address to bind to. Please ensure that the hostname and port are \
         correct",
    )
}

async fn create_shutdown_signal_listener() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to create shutdown signal handler");
}

pub async fn start(args: Args) -> eyre::Result<()> {
    let addr = create_socket_address(
        &args.server_config.grpc_hostname,
        args.server_config.grpc_port,
    )?;

    let (_health_reporter, health_service) = tonic_health::server::health_reporter();

    let reflection_service = if args.server_config.disable_grpc_reflection {
        None
    } else {
        Some(
            tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(crate::protos::FILE_DESCRIPTOR_SET)
                .build()
                .wrap_err("Failed to create gRPC reflection service")?,
        )
    };

    Server::builder()
        .add_service(health_service)
        .add_optional_service(reflection_service)
        .serve_with_shutdown(addr, create_shutdown_signal_listener())
        .await?;

    Ok(())
}

use units::runtime_v2;
use units::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config_path = std::env::args().nth(1);
    let config_path = config_path.ok_or(anyhow::anyhow!("config path is required"))?;

    let config = runtime_v2::RuntimeConfig::from_path(config_path.into())?;
    let server_config = config.server.clone();

    let runtime = runtime_v2::Runtime::init(config).await?;

    let grpc_server = server::Server::init(runtime)?;

    grpc_server.start(server_config).await?;

    Ok(())
}

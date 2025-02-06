pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("finternet_descriptor");

use tokio::signal::unix::{signal, SignalKind};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

use crate::runtime_v2;

pub struct Server {
    runtime: runtime_v2::Runtime,
}

impl Server {
    pub fn init(runtime: runtime_v2::Runtime) -> anyhow::Result<Self> {
        Ok(Self { runtime })
    }

    pub async fn start(self, config: runtime_v2::types::ServerConfig) -> anyhow::Result<()> {
        let socket_addr = std::net::SocketAddr::new(config.host.parse()?, config.port);

        tracing::info!(?socket_addr, "Starting server");

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build_v1()?;

        let (tx, rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            let mut sig_int = signal(SignalKind::interrupt())
                .expect("Failed to initialize SIGINT signal handler");
            let mut sig_term = signal(SignalKind::terminate())
                .expect("Failed to initialize SIGTERM signal handler");
            let mut sig_quit =
                signal(SignalKind::quit()).expect("Failed to initialize QUIT signal handler");
            let mut sig_hup =
                signal(SignalKind::hangup()).expect("Failed to initialize SIGHUP signal handler");

            tokio::select! {
                _ = sig_int.recv() => {
                    tracing::warn!("Received SIGINT");
                    tx.send(()).expect("Failed to send SIGINT signal");
                }
                _ = sig_term.recv() => {
                    tracing::warn!("Received SIGTERM");
                    tx.send(()).expect("Failed to send SIGTERM signal");
                }
                _ = sig_quit.recv() => {
                    tracing::warn!("Received QUIT");
                    tx.send(()).expect("Failed to send QUIT signal");
                }
                _ = sig_hup.recv() => {
                    tracing::warn!("Received SIGHUP");
                    tx.send(()).expect("Failed to send SIGHUP signal");
                }
            }
        });
        let shut_down_signal = async {
            rx.await.expect("Failed to receive shutdown signal");
            tracing::warn!("Shutdown signal received");
        };

        let health_service =
            super::health::proto_types::HealthServer::new(crate::health::HealthCheck);

        let execution_service = super::service::proto_types::execution_server::ExecutionServer::new(
            self.runtime.clone(),
        );
        let bind_service =
            super::service::proto_types::bind_server::BindServer::new(self.runtime.clone());
        let driver_service =
            super::service::proto_types::driver_server::DriverServer::new(self.runtime.clone());

        let driver_details_service =
            super::service::proto_types::driver_details_server::DriverDetailsServer::new(
                self.runtime.clone(),
            );

        let user_sign_up_service=
            super::service::proto_types::user_sign_up_server::UserSignUpServer::new(
                self.runtime.clone(),
            );

        let user_login_sevice=
            super::service::proto_types::user_login_server::UserLoginServer::new(
                self.runtime.clone(),
            );


        tonic::transport::Server::builder()
            .accept_http1(true)
            .layer(CorsLayer::permissive()) // Handle CORS
            .layer(GrpcWebLayer::new())
            .add_service(reflection_service)
            .add_service(health_service)
            .add_service(execution_service)
            .add_service(bind_service)
            .add_service(driver_service)
            .add_service(driver_details_service)
            .add_service(user_sign_up_service)
            .add_service(user_login_sevice)
            .serve_with_shutdown(socket_addr, shut_down_signal)
            .await
            .map_err(Into::into)
    }
}

use tonic::{Request, Response, Status};

pub struct HealthCheck;

pub mod proto_types {
    pub mod proto_items {
        tonic::include_proto!("grpc.health.v1");
    }

    pub use proto_items::{
        health_check_response::ServingStatus,
        health_server::{Health, HealthServer},
        HealthCheckRequest, HealthCheckResponse,
    };
}

#[tonic::async_trait]
impl proto_types::Health for HealthCheck {
    async fn check(
        &self,
        request: Request<proto_types::HealthCheckRequest>,
    ) -> Result<Response<proto_types::HealthCheckResponse>, Status> {
        tracing::debug!("Received health_check request: {:?}", request);

        let resp = proto_types::HealthCheckResponse {
            #[allow(clippy::as_conversions)]
            status: proto_types::ServingStatus::Serving as i32,
        };

        Ok(Response::new(resp))
    }
}

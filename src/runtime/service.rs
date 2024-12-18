use tonic::{Request, Response};

mod server_traits {
    pub use crate::service::proto_types::{
        bind_server::Bind, driver_server::Driver, execution_server::Execution,
    };
}

mod types {
    pub use crate::service::proto_types::{BindRequest, BindResponse};
    pub use crate::service::proto_types::{ExecutionRequest, ExecutionResponse};
    pub use crate::service::proto_types::{LoadDriverRequest, LoadDriverResponse};
    pub use crate::service::proto_types::{UnbindRequest, UnbindResponse};
    pub use crate::service::proto_types::{UnloadDriverRequest, UnloadDriverResponse};
}

#[tonic::async_trait]
impl server_traits::Execution for super::Runtime {
    async fn execute(
        &self,
        _request: Request<types::ExecutionRequest>,
    ) -> Result<Response<types::ExecutionResponse>, tonic::Status> {
        todo!()
    }
}

#[tonic::async_trait]
impl server_traits::Bind for super::Runtime {
    async fn bind(
        &self,
        _request: Request<types::BindRequest>,
    ) -> Result<Response<types::BindResponse>, tonic::Status> {
        todo!()
    }

    async fn unbind(
        &self,
        _request: Request<types::UnbindRequest>,
    ) -> Result<Response<types::UnbindResponse>, tonic::Status> {
        todo!()
    }
}

#[tonic::async_trait]
impl server_traits::Driver for super::Runtime {
    async fn load_driver(
        &self,
        _request: Request<types::LoadDriverRequest>,
    ) -> Result<Response<types::LoadDriverResponse>, tonic::Status> {
        todo!()
    }

    async fn unload_driver(
        &self,
        _request: Request<types::UnloadDriverRequest>,
    ) -> Result<Response<types::UnloadDriverResponse>, tonic::Status> {
        todo!()
    }
}

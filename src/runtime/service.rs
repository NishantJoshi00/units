use tonic::{Request, Response};

use crate::service::proto_types::BinaryType;

use super::Runtime;

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
        request: Request<types::ExecutionRequest>,
    ) -> Result<Response<types::ExecutionResponse>, tonic::Status> {
        let request = request.into_inner();
        let output =
            execte(self.clone(), request).map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(Response::new(output))
    }
}

fn execte(
    runtime: Runtime,
    request: types::ExecutionRequest,
) -> anyhow::Result<types::ExecutionResponse> {
    let module = match request.r#type() {
        BinaryType::Wat | BinaryType::Wasm => {
            wasmtime::Module::new(&runtime.process_layer.engine, request.binary)?
        }
    };

    let output = runtime.exec(module, request.input)?;

    Ok(types::ExecutionResponse { output })
}

#[tonic::async_trait]
impl server_traits::Bind for super::Runtime {
    async fn bind(
        &self,
        request: Request<types::BindRequest>,
    ) -> Result<Response<types::BindResponse>, tonic::Status> {
        let mut writer = self
            .driver_layer
            .resolver
            .mount_points
            .write()
            .map_err(|_| tonic::Status::internal("Failed to lock mount points".to_string()))?;

        let request = request.into_inner();
        writer.insert(
            request.path.clone(),
            (request.driver_name.clone(), request.account_info.clone()),
        );

        let output = types::BindResponse {
            driver_name: request.driver_name,
            account_info: request.account_info,
            path: request.path,
        };

        Ok(Response::new(output))
    }

    async fn unbind(
        &self,
        request: Request<types::UnbindRequest>,
    ) -> Result<Response<types::UnbindResponse>, tonic::Status> {
        let mut writer = self
            .driver_layer
            .resolver
            .mount_points
            .write()
            .map_err(|_| tonic::Status::internal("Failed to lock mount points".to_string()))?;

        let request = request.into_inner();
        let output = writer.remove(&request.path);

        match output {
            None => Err(tonic::Status::not_found("Path not found")),
            Some((driver_name, account_info)) => Ok(Response::new(types::UnbindResponse {
                driver_name,
                account_info,
            })),
        }
    }
}

#[tonic::async_trait]
impl server_traits::Driver for super::Runtime {
    async fn load_driver(
        &self,
        request: Request<types::LoadDriverRequest>,
    ) -> Result<Response<types::LoadDriverResponse>, tonic::Status> {
        let request = request.into_inner();

        let module = match request.driver_type() {
            BinaryType::Wat | BinaryType::Wasm => {
                wasmtime::Module::new(&self.driver_layer.engine, request.driver_binary)
                    .map_err(|e| tonic::Status::internal(e.to_string()))?
            }
        };

        self.driver_layer
            .add_driver(request.driver_name.clone(), module)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(types::LoadDriverResponse {
            driver_name: request.driver_name,
            driver_version: request.driver_version,
        }))
    }

    async fn unload_driver(
        &self,
        request: Request<types::UnloadDriverRequest>,
    ) -> Result<Response<types::UnloadDriverResponse>, tonic::Status> {
        let request = request.into_inner();

        self.driver_layer
            .remove_driver(request.driver_name.clone())
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(types::UnloadDriverResponse {
            driver_name: request.driver_name,
        }))
    }
}

use tonic::{Request, Response};

use crate::runtime_v2::driver::DriverInfo;
use crate::runtime_v2::types::ProcessState;
use crate::runtime_v2::types::UserCtx;
use crate::service::proto_types::DriverDetail;

use super::Runtime;

mod server_traits {
    pub use crate::service::proto_types::{
        bind_server::Bind,
        driver_details_server::DriverDetails, // for driver details
        driver_server::Driver,
        execution_server::Execution,
    };
}

mod types {
    pub use crate::service::proto_types::{BindRequest, BindResponse};
    pub use crate::service::proto_types::{DriverDetailsRequest, DriverDetailsResponse};
    pub use crate::service::proto_types::{ExecutionRequest, ExecutionResponse};
    pub use crate::service::proto_types::{ListProgramRequest, ListProgramResponse, Program};
    pub use crate::service::proto_types::{ListResolverRequest, ListResolverResponse, PathMapping};
    pub use crate::service::proto_types::{LoadDriverRequest, LoadDriverResponse};
    pub use crate::service::proto_types::{SubmitProgramRequest, SubmitProgramResponse};
    pub use crate::service::proto_types::{UnbindRequest, UnbindResponse};
    pub use crate::service::proto_types::{UnloadDriverRequest, UnloadDriverResponse};
    //for sending all driver details
}

#[tonic::async_trait]
impl server_traits::Execution for super::Runtime {
    async fn execute(
        &self,
        request: Request<types::ExecutionRequest>,
    ) -> Result<Response<types::ExecutionResponse>, tonic::Status> {
        let request = request.into_inner();
        let output = execte(self.clone(), request)
            .await
            .inspect_err(|err| {
                tracing::error!(error = ?err, "Execution failed");
            })
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(Response::new(output))
    }
    async fn submit(
        &self,
        request: Request<types::SubmitProgramRequest>,
    ) -> Result<Response<types::SubmitProgramResponse>, tonic::Status> {
        let request = request.into_inner();
        let component =
            wasmtime::component::Component::new(&self.process_layer.engine, request.binary)
                .map_err(|e| tonic::Status::internal(e.to_string()))?;
        let id = self
            .process_layer
            .store_program(request.name, request.version, component)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(types::SubmitProgramResponse {
            program_id: id,
        }))
    }

    async fn list(
        &self,
        _request: Request<types::ListProgramRequest>,
    ) -> Result<Response<types::ListProgramResponse>, tonic::Status> {
        Ok(Response::new(types::ListProgramResponse {
            program: self
                .process_layer
                .programs
                .list(self.process_layer.engine.clone())
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?
                .into_iter()
                .map(|(id, program)| types::Program {
                    program_id: id.clone(),
                    name: program.name.clone(),
                    version: program.version.clone(),
                })
                .collect(),
        }))
    }
}

async fn execte(
    runtime: Runtime,
    request: types::ExecutionRequest,
) -> anyhow::Result<types::ExecutionResponse> {
    let component = match (request.program_id, request.binary) {
        (Some(program_id), None) => runtime
            .process_layer
            .find_program(&program_id, runtime.process_layer.engine.clone())
            .await?
            .map(|prog| prog.component)
            .ok_or_else(|| anyhow::anyhow!("Program not found"))?,
        (None, Some(binary)) => {
            wasmtime::component::Component::new(&runtime.process_layer.engine, binary)?
        }
        _ => {
            anyhow::bail!("Either program_id or binary should be provided (but not both)")
        }
    };

    let output = runtime
        .exec(
            super::types::UserCtx {
                user_id: "root".to_string(),
            },
            component,
            request.input,
        )
        .await?;

    Ok(types::ExecutionResponse { output })
}

#[tonic::async_trait]
impl server_traits::Bind for super::Runtime {
    async fn bind(
        &self,
        request: Request<types::BindRequest>,
    ) -> Result<Response<types::BindResponse>, tonic::Status> {
        let request = request.into_inner();

        let mut process_state = ProcessState::new(
            UserCtx {
                user_id: "root".to_string(),
            },
            self.driver_layer.clone(),
            self.platform_layer.clone(),
            self.event_sender.clone(),
        );

        process_state
            .perform_bind(
                request.path.clone(),
                DriverInfo {
                    name: request.driver_name.clone(),
                    version: request.driver_version.clone(),
                },
                request.account_info.clone(),
            )
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let output = types::BindResponse {
            driver_name: request.driver_name,
            driver_version: request.driver_version,
            account_info: request.account_info,
            path: request.path,
        };

        Ok(Response::new(output))
    }

    async fn unbind(
        &self,
        request: Request<types::UnbindRequest>,
    ) -> Result<Response<types::UnbindResponse>, tonic::Status> {
        let request = request.into_inner();
        let output = self.driver_layer.resolver.remove(&request.path).await;

        match output {
            None => Err(tonic::Status::not_found("Path not found")),
            Some(path_info) => Ok(Response::new(types::UnbindResponse {
                driver_name: path_info.driver_name,
                driver_version: path_info.driver_version,
                account_info: path_info.account_info,
            })),
        }
    }
}

#[tonic::async_trait]
impl server_traits::Driver for super::Runtime {
    async fn list_resolver(
        &self,
        _request: Request<types::ListResolverRequest>,
    ) -> Result<Response<types::ListResolverResponse>, tonic::Status> {
        let output =
            self.driver_layer
                .resolver
                .list()
                .await
                .into_iter()
                .map(|(path, path_info)| {
                    let path = path.clone();
                    let path_info = path_info.clone();
                    types::PathMapping {
                        path,
                        driver_name: path_info.driver_name,
                        driver_version: path_info.driver_version,
                        account_info: path_info.account_info,
                    }
                });

        Ok(Response::new(types::ListResolverResponse {
            path_mapping: output.collect(),
        }))
    }
    async fn load_driver(
        &self,
        request: Request<types::LoadDriverRequest>,
    ) -> Result<Response<types::LoadDriverResponse>, tonic::Status> {
        let request = request.into_inner();

        tracing::info!(name = %request.driver_name,version=%request.driver_version, "Adding driver");

        let module =
            wasmtime::component::Component::new(&self.driver_layer.engine, request.driver_binary)
                .map_err(|e| tonic::Status::internal(e.to_string()))?;

        tracing::info!(name = ?request.driver_name, "Module Created");

        self.driver_layer
            .add_driver(
                request.driver_name.clone(),
                module,
                request.driver_version.clone(),
            )
            .await
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

        let driver_info = DriverInfo {
            name: request.driver_name.clone(),
            version: request.driver_version.clone(),
        };

        self.driver_layer
            .remove_driver(driver_info)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(types::UnloadDriverResponse {
            driver_name: request.driver_name,
            driver_version: request.driver_version,
        }))
    }
}

#[tonic::async_trait]
impl server_traits::DriverDetails for super::Runtime {
    async fn send_details(
        &self,
        _request: Request<types::DriverDetailsRequest>,
    ) -> Result<Response<types::DriverDetailsResponse>, tonic::Status> {
        let mut all_driver_details = Vec::<DriverDetail>::new();
        let mut message = String::from("Drivers Detail list found!!");
        let reader = &self.driver_layer.drivers;
        for (driver_info, _module) in reader
            .list(self.driver_layer.engine.clone())
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?
        {
            let new_driver = DriverDetail {
                name: driver_info.name.clone(),
                version: driver_info.version.clone(),
            };
            all_driver_details.push(new_driver);
        }

        if all_driver_details.is_empty() {
            message = String::from("Driver Details not found!!")
        }

        Ok(tonic::Response::new(types::DriverDetailsResponse {
            message,
            driver_data: all_driver_details,
        }))
    }
}

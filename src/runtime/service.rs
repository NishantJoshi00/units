use tonic::{Request, Response};

use crate::runtime::driver::DriverInfo;
use crate::runtime::resolver::PathInfo;
use crate::service::proto_types::BinaryType;
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
    pub use crate::service::proto_types::{LoadDriverRequest, LoadDriverResponse};
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
            .inspect_err(|err| {
                tracing::error!(error = ?err, "Execution failed");
            })
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
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

        let reader = self
            .driver_layer
            .drivers
            .read()
            .map_err(|_| tonic::Status::internal("Failed to lock drivers".to_string()))?;

        let driver_info = DriverInfo {
            name: request.driver_name.clone(),
            version: request.driver_version.clone(),
        };

        if !reader.contains_key(&driver_info) {
            tracing::error!(name = %request.driver_name,version=%request.driver_version, "Driver not found");
            return Err(tonic::Status::not_found("Driver not found"));
        }

        let path_info = PathInfo {
            driver_name: request.driver_name.clone(),
            driver_version: request.driver_version.clone(),
            account_info: request.account_info.clone(),
        };

        writer.insert(request.path.clone(), path_info);

        tracing::info!(path = %request.path, driver = %request.driver_name,verion=%request.driver_version ,account_info = %request.account_info, "Path bound");

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
    async fn load_driver(
        &self,
        request: Request<types::LoadDriverRequest>,
    ) -> Result<Response<types::LoadDriverResponse>, tonic::Status> {
        let request = request.into_inner();

        tracing::info!(name = %request.driver_name,version=%request.driver_version, "Adding driver");

        let module = match request.driver_type() {
            BinaryType::Wat | BinaryType::Wasm => {
                wasmtime::Module::new(&self.driver_layer.engine, request.driver_binary)
                    .map_err(|e| tonic::Status::internal(e.to_string()))?
            }
        };

        tracing::info!(name = ?module.name(), "Module Created");

        self.driver_layer
            .add_driver(
                request.driver_name.clone(),
                module,
                request.driver_version.clone(),
            )
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
        let locked_map = reader
            .read()
            .map_err(|_| tonic::Status::internal("Failed to lock map"))?;
        for (driver_info, _module) in locked_map.iter() {
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

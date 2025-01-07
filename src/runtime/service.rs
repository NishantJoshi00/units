use tonic::{Request, Response};

use crate::service::proto_types::BinaryType;
use crate::service::proto_types::DriverDetail; // Use this if you need the proto version.


use super::Runtime;

mod server_traits {
    pub use crate::service::proto_types::{
        bind_server::Bind, driver_server::Driver, execution_server::Execution,
        driver_details_server::DriverDetails// for driver details
    };
}

mod types {
    pub use crate::service::proto_types::{BindRequest, BindResponse};
    pub use crate::service::proto_types::{ExecutionRequest, ExecutionResponse};
    pub use crate::service::proto_types::{LoadDriverRequest, LoadDriverResponse};
    pub use crate::service::proto_types::{UnbindRequest, UnbindResponse};
    pub use crate::service::proto_types::{UnloadDriverRequest, UnloadDriverResponse};
    pub use crate::service::proto_types::{DriverDetailsRequest,DriverDetailsResponse};//for sending all driver details
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

        if !reader.contains_key(&request.driver_name) {
            tracing::error!(name = %request.driver_name, "Driver not found");
            return Err(tonic::Status::not_found("Driver not found"));
        }

        writer.insert(
            request.path.clone(),
            (request.driver_name.clone(), request.account_info.clone()),
        );

        tracing::info!(path = %request.path, driver = %request.driver_name, account_info = %request.account_info, "Path bound");

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

        tracing::info!(name = %request.driver_name, "Adding driver");

        let module = match request.driver_type() {
            BinaryType::Wat | BinaryType::Wasm => {
                wasmtime::Module::new(&self.driver_layer.engine, request.driver_binary)
                    .map_err(|e| tonic::Status::internal(e.to_string()))?
            }
        };

        tracing::info!(name = ?module.name(), "Module Created");

        self.driver_layer
            .add_driver(request.driver_name.clone(), module,request.driver_version.clone())
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



#[tonic::async_trait]
impl server_traits::DriverDetails for super::Runtime{
    async fn send_details(
        &self,
        _request: Request<types::DriverDetailsRequest>,
    )-> Result<Response<types::DriverDetailsResponse>, tonic::Status> {

        
        let mut all_driver_details = Vec::<DriverDetail>::new();
        let reader=&self.driver_layer.drivers;
        let locked_map = reader.read().map_err(|_| tonic::Status::internal("Failed to lock map"))?;
        for (key, driver_info) in locked_map.iter() {
                let version=driver_info.version.clone();
                let new_driver=DriverDetail{
                    name:key.clone(),
                    version,
                };
            all_driver_details.push(new_driver);
        }


        Ok(tonic::Response::new(types::DriverDetailsResponse{
            success:true,
            driver_data:all_driver_details,
        }))
    }
}


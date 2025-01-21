use tonic::{Request,Status, Response};
use tonic::metadata::MetadataValue;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::runtime::driver::DriverInfo;
use crate::runtime::resolver::PathInfo;
use crate::service::proto_types::BinaryType;
use crate::service::proto_types::DriverDetail;

use super::Runtime;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String, 
    exp: usize,   
    iat: usize,   
}

mod server_traits {
    pub use crate::service::proto_types::{
        bind_server::Bind,
        driver_details_server::DriverDetails, 
        driver_server::Driver,
        execution_server::Execution,
        user_sign_up_server::UserSignUp,
        user_login_server::UserLogin,
    };
}

mod types {
    pub use crate::service::proto_types::{BindRequest, BindResponse};
    pub use crate::service::proto_types::{DriverDetailsRequest, DriverDetailsResponse};
    pub use crate::service::proto_types::{ExecutionRequest, ExecutionResponse};
    pub use crate::service::proto_types::{LoadDriverRequest, LoadDriverResponse};
    pub use crate::service::proto_types::{UnbindRequest, UnbindResponse};
    pub use crate::service::proto_types::{UnloadDriverRequest, UnloadDriverResponse};
    pub use crate::service::proto_types::{SignUpRequest,SignUpResponse};
    pub use crate::service::proto_types::{LoginRequest,LoginResponse};
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
    let component = match request.r#type() {
            BinaryType::Wat | BinaryType::Wasm => {
                wasmtime::component::Component::new(&runtime.driver_layer.engine, request.binary)
                    .map_err(|e| tonic::Status::internal(e.to_string()))?
            }
        };

    let output = runtime.exec(component, request.input)?;

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

        let component = match request.driver_type() {
            BinaryType::Wat | BinaryType::Wasm => {
                wasmtime::component::Component::new(&self.driver_layer.engine, request.driver_binary)
                    .map_err(|e| tonic::Status::internal(e.to_string()))?
            }
        };

        // tracing::info!(name = ?component, " Created");

        self.driver_layer
            .add_driver(
                request.driver_name.clone(),
                component,
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

#[tonic::async_trait]
impl server_traits::UserSignUp for super::Runtime {
    async fn sign_up(
        &self,
        request: Request<types::SignUpRequest>,
    ) -> Result<Response<types::SignUpResponse>, tonic::Status> {
        let request = request.into_inner();
        self.driver_layer
            .add_user(
                request.username.clone(),
                request.name.clone(),
                request.password.clone(),
                request.email.clone(),
            )
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        let message=format!("{} has signed up successfully",request.username.clone());
        Ok(tonic::Response::new(types::SignUpResponse {
            message,
        }))
    }
}

#[tonic::async_trait]
impl server_traits::UserLogin for super::Runtime {
    async fn login(
        &self,
        request: Request<types::LoginRequest>,
    ) -> Result<Response<types::LoginResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut hasher = blake3::Hasher::new();
        hasher.update(request.password.as_bytes());
        let hash_pass = hasher.finalize();

        let read_guard = self
            .driver_layer
            .user_store
            .read()
            .map_err(|_| Status::internal("Failed to lock map"))?;
        
        let (message, set_cookie) = match read_guard.get(&request.username) {
            Some(user) => {
                if hash_pass == user.password {
                    let expiration = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as usize + 60; 

                    let claims = Claims {
                        username: request.username.clone(),
                        exp: expiration,
                        iat: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as usize,
                    };

                    let secret = b"finternet";
                    
                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(secret)
                    ).map_err(|_| Status::internal("Failed to create token"))?;

                    let cookie = format!(
                        "auth_token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Strict",
                        token
                    );
                    
                    (
                        format!("{} has logged in successfully", request.username),
                        Some(cookie)
                    )
                } else {
                    (String::from("Password is incorrect"), None)
                }
            }
            None => (String::from("User not found"), None),
        };

        let mut response = Response::new(types::LoginResponse { message });
        
        if let Some(cookie_value) = set_cookie {
            response.metadata_mut().insert(
                "set-cookie",
                MetadataValue::try_from(&cookie_value)
                    .map_err(|_| Status::internal("Failed to create cookie metadata"))?
            );
        }

        Ok(response)
    }
}

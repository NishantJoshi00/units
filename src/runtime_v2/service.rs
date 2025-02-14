use tonic::{Request,Status, Response};
use tonic::metadata::MetadataValue;
use jsonwebtoken::{encode, decode, EncodingKey, Header, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;
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
        user_sign_up_server::UserSignUp,
        user_login_server::UserLogin,
        user_check_server::UserCheck,
    };
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: String,
    username: String, 
    exp: usize,   
    iat: usize,   
}

struct UserData{
    message: bool,
    username:String,
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
    pub use crate::service::proto_types::{SignUpRequest,SignUpResponse};
    pub use crate::service::proto_types::{LoginRequest,LoginResponse};
    pub use crate::service::proto_types::{CheckRequest,CheckResponse};
}

fn check_jwt<T>(request: &Request<T>) -> Result<UserData, Box<dyn Error>> {
    let token = match request.metadata().get("Authorization") {
        Some(token) => token.to_str()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Invalid token".to_string()),
        None => return Ok(UserData{
            message: false,
            username: "".to_string(),
        }),
    };

    let secret = env::var("secret").unwrap_or_else(|_| "default_value".to_string());
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    match claims {
        Ok(claims) => {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if claims.claims.exp < current_time as usize {
                return Ok(UserData{
                    message: false,
                    username: "".to_string(),
                }); // Expired token
            } else {
                return Ok(UserData{
                    message: true,
                    username: claims.claims.username.to_string(),
                }); // Return username if valid
            }
        }
        Err(_err) => {
            return Ok(UserData{
                message: false,
                username: "".to_string(),
            });
        }
    }
}
fn get_user_id<T>(request:&Request<T>)->Result<String,Box<dyn Error>>{
    let token = match request.metadata().get("Authorization") {
        Some(token) => token.to_str()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Invalid token".to_string()),
        None => return Err(Box::new(Status::unauthenticated("No JWT token found"))),
    };
    
    let secret = env::var("secret").unwrap_or_else(|_| "default_value".to_string());
    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    match claims {
        Ok(claims) => {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(); 
            if claims.claims.exp < current_time as usize {
                return Err("Invalid or expired token".to_string().into());// Expired token
            } else {
                return Ok(claims.claims.user_id.to_string()); // Return user ID if valid
            }
        }
        Err(_err) => {
            return Err("Invalid or expired token".to_string().into());
        }
    };
}

#[tonic::async_trait]
impl server_traits::Execution for super::Runtime {
    async fn execute(
        &self,
        request: Request<types::ExecutionRequest>,
    ) -> Result<Response<types::ExecutionResponse>, tonic::Status> {
        let user_id= get_user_id(&request).map_err(|e| tonic::Status::internal(e.to_string()))?;
        let request = request.into_inner();
        let output = execte(self.clone(), request, user_id)
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
    user_id: String,
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

    let output = runtime.exec(
        super::types::UserCtx {
            user_id: user_id.to_string(),
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
        let user_id= get_user_id(&request).map_err(|e| tonic::Status::internal(e.to_string()))?;
        let request = request.into_inner();
        let mut process_state = ProcessState::new(
            UserCtx {
                user_id:user_id.to_string(),
            },
            self.driver_layer.clone(),
            self.platform_layer.clone(),
            self.event_sender.clone(),
        );

        let path = if let Some(suffix) = request.path.strip_prefix("~/") {
            format!("/accounts/{}/{}",user_id,suffix)
        } else {
            request.path.clone()
        };
        
        
        process_state
            .perform_bind(
                path,
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

#[tonic::async_trait]
impl server_traits::UserSignUp for super::Runtime {
    async fn sign_up(
        &self,
        request: Request<types::SignUpRequest>,
    ) -> Result<Response<types::SignUpResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut hasher = blake3::Hasher::new();
        hasher.update(request.password.as_bytes()); 
        let hash_pass = hasher.finalize(); 
        
        self.driver_layer.user.insert(&request.username, &hash_pass.to_string())
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        let message=format!("{} has signed up successfully",request.username);
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

        let user=self.driver_layer.user.get(&request.username, &hash_pass.to_string()).await;
        
        let (message, set_cookie) = match user {
            Ok(None) => (String::from("User not found"), None),
            Ok(Some(user)) => {
                    let expiration = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as usize + 3600; 
                    let claims = Claims {
                        username: request.username.clone(),
                        user_id: user,
                        exp: expiration,
                        iat: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as usize,
                    };
                    let secret = env::var("secret").unwrap_or_else(|_| "default_value".to_string());
                    
                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(secret.as_bytes()),
                    ).map_err(|_| Status::internal("Failed to create token"))?;
        
                    let cookie = format!(
                        "{}",
                        token
                    );
                    
                    (
                        cookie.to_string(),
                        Some(cookie)
                    )
                } 
            Err(_) => (String::from("Error retrieving user"), None),
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

#[tonic::async_trait]
impl server_traits::UserCheck for super::Runtime {
    async fn check(
        &self,
        request: Request<types::CheckRequest>,
    ) -> Result<Response<types::CheckResponse>, tonic::Status> {
        let user_data=check_jwt(&request).map_err(|e| tonic::Status::internal(e.to_string()))?;
        
        Ok(tonic::Response::new(types::CheckResponse {
            message: user_data.message,
            username: user_data.username,
        }))
    }
}

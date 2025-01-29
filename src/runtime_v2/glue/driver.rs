use crate::runtime_v2::types::component::driver::component::units::storage::{Host, StorageError};
use crate::runtime_v2::types::DriverState;

impl Host for DriverState {
    fn get(&mut self, key: String) -> Result<String, StorageError> {
        tracing::info!(runtime = "driver", call = "get", key = key.as_str());
        let output =
            self.platform.storage.get(&key).map_err(|e| {
                StorageError::SystemError(format!("Failed while getting key: {:?}", e))
            })?;
        match output {
            Some(value) => Ok(value),
            None => Err(StorageError::NotFound(format!("Key not found: {}", key))),
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<(), StorageError> {
        tracing::info!(runtime = "driver", call = "set", key = key.as_str());
        self.platform
            .storage
            .set(&key, &value)
            .map_err(|e| StorageError::SystemError(format!("Failed while setting key: {:?}", e)))
    }
}


mod http_impl {

    use surf::Status;

    use crate::runtime_v2::types::DriverState;
    use crate::runtime_v2::types::{component::driver::component::units::*};

    impl http::Host for DriverState {
        fn send_request(&mut self, request: http::Request) -> http::Response {
            let output = match request.method {
                http::Method::Get => {
                    let mut urequest = ureq::get(&request.url);
                    for (key, value) in request.headers.iter() {
                        urequest = urequest.header(key, value);
                    }
                    urequest.call()
                }
                http::Method::Post => {

                    let mut urequest = ureq::post(&request.url);
                    for (key, value) in request.headers.iter() {
                        urequest = urequest.header(key, value);
                    }
                    let body = request.body.expect("failed to get body");
                    urequest.send(&body)
                }
                http::Method::Put => {
                    let mut urequest = ureq::put(&request.url);
                    for (key, value) in request.headers.iter() {
                        urequest = urequest.header(key, value);
                    }
                    let body = request.body.expect("failed to get body");
                    urequest.send(&body)
                }
                http::Method::Delete => {
                    let mut urequest = ureq::put(&request.url);
                    for (key, value) in request.headers.iter() {
                        urequest = urequest.header(key, value);
                    }
                    let body = request.body.expect("failed to get body");
                    urequest.send(&body)
                }
            };

            let response = output.expect("failed to send request");

            let response = http::Response {
                status: response.status().as_u16(),
                headers: response.headers().iter().map(|(name, value)| {
                    (
                        name.as_str().to_string(),
                        value.to_str().expect("failed to convert header value").to_string(),
                        )
                }).collect(),
                body: response.into_body().read_to_string().expect("failed to read body"),
            };

            response


        }
    }
}

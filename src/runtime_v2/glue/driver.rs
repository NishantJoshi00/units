use crate::runtime_v2::types::component::driver::component::units::storage::{Host, StorageError};
use crate::runtime_v2::types::DriverState;

impl Host for DriverState {
    async fn get(&mut self, key: String) -> Result<String, StorageError> {
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

    async fn set(&mut self, key: String, value: String) -> Result<(), StorageError> {
        tracing::info!(runtime = "driver", call = "set", key = key.as_str());
        self.platform
            .storage
            .set(&key, &value)
            .map_err(|e| StorageError::SystemError(format!("Failed while setting key: {:?}", e)))
    }
}

mod http_impl {
    use once_cell::sync::Lazy;
    static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

    use crate::runtime_v2::types::component::driver::component::units::*;
    use crate::runtime_v2::types::DriverState;

    impl http::Host for DriverState {

        async fn send_request(&mut self, request: http::Request) -> http::Response {
            // Clone the client first to avoid any potential thread contention
            let client = HTTP_CLIENT.clone();

            let response = match request.method {
                http::Method::Get => {
                    let mut req_builder = client.get(&request.url);
                    for (key, value) in request.headers.iter() {
                        req_builder = req_builder.header(key, value);
                    }
                    req_builder.send().await
                }
                http::Method::Post => {
                    let mut req_builder = client.post(&request.url);
                    for (key, value) in request.headers.iter() {
                        req_builder = req_builder.header(key, value);
                    }
                    if let Some(body) = request.body {
                        req_builder = req_builder.body(body);
                    }
                    req_builder.send().await
                }
                http::Method::Put => {
                    let mut req_builder = client.put(&request.url);
                    for (key, value) in request.headers.iter() {
                        req_builder = req_builder.header(key, value);
                    }
                    if let Some(body) = request.body {
                        req_builder = req_builder.body(body);
                    }
                    req_builder.send().await
                }
                http::Method::Delete => {
                    let mut req_builder = client.delete(&request.url);
                    for (key, value) in request.headers.iter() {
                        req_builder = req_builder.header(key, value);
                    }
                    if let Some(body) = request.body {
                        req_builder = req_builder.body(body);
                    }
                    req_builder.send().await
                }
            };

            let response = response.expect("failed to send request");

            let headers = response
                .headers()
                .iter()
                .map(|(name, value)| {
                    (
                        name.as_str().to_string(),
                        value
                            .to_str()
                            .expect("failed to convert header value")
                            .to_string(),
                    )
                })
                .collect();

            let status = response.status().as_u16();
            let body = response.text().await.expect("failed to read body");

            http::Response {
                status,
                headers,
                body,
            }
        }
    }
}

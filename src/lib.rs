pub mod dispatcher;
pub mod health;
pub mod runtime;
pub mod server;
pub mod service;
pub mod types;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = anyhow::Error;

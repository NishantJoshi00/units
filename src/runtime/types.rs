use core::marker::PhantomData;

mod formatter;
mod impls;

///
/// [`RType`] is a generic type that is used to represent different types on the system.
///
pub struct RType<Format = formatter::Json>(Format::SType, PhantomData<Format>)
where
    Format: formatter::Formatter;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub runtime: RuntimeConfig,
    pub driver: DriverConfig,
    pub process: ProcessConfig,
    pub platform: PlatformConfig,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RuntimeConfig {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DriverConfig {
    pub driver_limit: u32,
    pub driver_timeout: u32, // in seconds
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProcessConfig {}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PlatformConfig {}


#[derive(Debug, Clone, serde::Deserialize)]
pub struct Event {
    pub loc: Loc,
    pub event_type: EventType,
    pub level: Level,
    pub call_type: CallType,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum Level {
    Platform,
    Driver
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum CallType {
    Intend,
    Done,
    Transfer,
    View,
    Set,
    Get
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub enum EventType {
    #[default]
    Info,
    Error,
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub enum Loc {
    Start,
    #[default]
    Event,
    End,
}

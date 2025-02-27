use anyhow::anyhow;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use starknet::macros::selector;
use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use tracing_subscriber::fmt::format;
use wasmtime::component::Component;

// use wasmtime_wasi::preview1::WasiP1Ctx;
use crate::runtime_v2::provable;
use crate::runtime_v2::provable::ProvableRuntime;
use wasmtime_wasi::{WasiCtx, WasiView};

use super::driver::{self, DriverInfo};
use super::platform::Platform;
use super::resolver::PathInfo;

pub mod component {
    pub mod driver {
        wasmtime::component::bindgen!({
            world: "driver-world",
            path: "wit",
            tracing: true,
            async: true,
        });
    }

    pub mod module {
        wasmtime::component::bindgen!({
            world: "module-world",
            path: "wit",
            tracing: true,
            async: true,
        });
    }
}

#[derive(Clone)]
pub struct UserCtx {
    pub user_id: String,
}

#[derive(Clone)]
pub struct DriverCtx {
    pub driver_info: DriverInfo,
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
pub struct ProvableConfig {
    pub url: String,
    pub operator_private_key: String,
    pub operator_address: String,
    pub chain_id: String,
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
    Driver,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum CallType {
    Intend,
    Done,
    Transfer,
    View,
    Set,
    Get,
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

pub struct ProcessState {
    pub ctx: UserCtx,
    pub driver_runtime: driver::DriverRuntime,
    pub provable_runtime: ProvableRuntime,
    pub platform: Platform,
    pub event_sender: Arc<mpsc::Sender<Event>>,
    pub descriptors: HashMap<String, Descriptor>,
    // table: ResourceTable,
    // wasi_ctx: WasiCtx,
}

#[derive(Clone)]
pub struct Descriptor {
    pub driver_name: String,
    pub driver_version: String,
    pub account_info: serde_json::Value,
}

pub struct DriverState {
    pub ctx: UserCtx,
    pub driver_ctx: DriverCtx,
    pub platform: Platform,
    pub event_sender: Arc<mpsc::Sender<Event>>,
    // table: ResourceTable,
    // wasi_ctx: WasiCtx,
}

#[derive(Deserialize)]
pub struct CairoInput {
    pub program_input: String,
    pub user_signature: String,
}

impl ProcessState {
    pub fn new(
        ctx: UserCtx,
        driver_runtime: driver::DriverRuntime,
        platform: Platform,
        event_sender: Arc<mpsc::Sender<Event>>,
        provable_runtime: provable::ProvableRuntime,
    ) -> Self {
        Self {
            ctx,
            driver_runtime,
            platform,
            event_sender,
            descriptors: HashMap::new(),
            provable_runtime, // table: ResourceTable::new(),
                              // wasi_ctx: wasmtime_wasi::WasiCtxBuilder::new().build(),
        }
    }

    pub async fn get_path_info(
        &self,
        input: String,
    ) -> Result<super::resolver::PathInfo, component::module::component::units::driver::DriverError>
    {
        let path_info = self
            .driver_runtime
            .resolver
            .get(input.as_str())
            .await
            .ok_or(
                component::module::component::units::driver::DriverError::InvalidInput(
                    "Failed while resolving path".to_string(),
                ),
            )?;
        Ok(path_info)
    }

    pub async fn get_driver(
        &self,
        input: &DriverInfo,
        engine: wasmtime::Engine,
    ) -> Result<ProgramComponent, component::module::component::units::driver::DriverError> {
        let driver = self
            .driver_runtime
            .drivers
            .get(input, engine)
            .await
            .map_err(|_| {
                component::module::component::units::driver::DriverError::InvalidInput(
                    "Failed while finding driver".to_string(),
                )
            })?;

        driver.clone().ok_or(
            component::module::component::units::driver::DriverError::InvalidInput(
                "Failed while finding driver".to_string(),
            ),
        )
    }

    pub fn get_lower_runtime(
        &self,
        driver_info: DriverInfo,
    ) -> Result<
        (
            wasmtime::component::Linker<DriverState>,
            wasmtime::Store<DriverState>,
        ),
        component::module::component::units::driver::DriverError,
    > {
        let state = wasmtime::Store::new(
            &self.driver_runtime.engine,
            DriverState::new(
                self.ctx.clone(),
                DriverCtx { driver_info },
                self.platform.clone(),
                self.event_sender.clone(),
            ),
        );
        let mut linker = wasmtime::component::Linker::new(&self.driver_runtime.engine);
        component::driver::DriverWorld::add_to_linker(&mut linker, |state: &mut DriverState| state)
            .map_err(|_| {
                component::module::component::units::driver::DriverError::SystemError(
                    "Failed while adding driver to linker".to_string(),
                )
            })?;
        // wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|e| {
        //     component::module::component::units::driver::DriverError::SystemError(e.to_string())
        // })?;

        // wasmtime_wasi_http::add_to_linker_sync(&mut linker).map_err(|e| {
        //     component::module::component::units::driver::DriverError::SystemError(e.to_string())
        // })?;

        Ok((linker, state))
    }

    pub fn get_descriptor(
        &self,
        key: String,
    ) -> Result<&Descriptor, component::module::component::units::driver::DriverError> {
        self.descriptors.get(&key).ok_or(
            component::module::component::units::driver::DriverError::InvalidInput(
                "Failed while finding descriptor".to_string(),
            ),
        )
    }

    pub fn delete_descriptor(
        &mut self,
        key: String,
    ) -> Result<(), component::module::component::units::driver::DriverError> {
        self.descriptors.remove(&key).ok_or(
            component::module::component::units::driver::DriverError::InvalidInput(
                "Failed while deleting descriptor".to_string(),
            ),
        )?;
        Ok(())
    }

    pub async fn perform_bind(
        &mut self,
        path: String,
        driver_info: DriverInfo,
        input: String,
    ) -> Result<(), component::module::component::units::driver::DriverError> {
        // valid driver :: check
        if self
            .driver_runtime
            .drivers
            .get(&driver_info, self.driver_runtime.engine.clone())
            .await
            .map_err(|_| {
                component::module::component::units::driver::DriverError::InvalidInput(
                    "Failed while finding driver".to_string(),
                )
            })?
            .is_none()
        {
            return Err(
                component::module::component::units::driver::DriverError::InvalidInput(
                    "Failed while finding driver".to_string(),
                ),
            );
        }

        let output = self.driver_runtime.resolver.get(path.as_str()).await;

        match output {
            None => {
                let driver = self
                    .get_driver(&driver_info, self.driver_runtime.engine.clone())
                    .await?;
                let (mut linker, mut state) = self.get_lower_runtime(driver_info.clone())?;

                wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|err| {
                    component::module::component::units::driver::DriverError::SystemError(
                        err.to_string(),
                    )
                })?;

                let output = match driver {
                    ProgramComponent::WASM(driver) => {
                        let instance = component::driver::DriverWorld::instantiate_async(
                            &mut state,
                            &driver.module,
                            &linker,
                        )
                        .await
                        .map_err(|e| {
                            component::module::component::units::driver::DriverError::SystemError(
                                e.to_string(),
                            )
                        })?;
                        instance
                            .component_units_driver()
                            .call_bind(state, &input, None)
                            .await
                            .map_err(|e| {
                                component::module::component::units::driver::DriverError::SystemError(
                                    e.to_string(),
                                )
                            })?
                            .map_err(|e| {
                                component::module::component::units::driver::DriverError::SystemError(
                                    e.to_string(),
                                )
                            })?
                    }
                    ProgramComponent::Cairo(driver) => {
                        // TODO: change error type of `perform_bind` to allow Cairo and WASM errors
                        let cairo_input: CairoInput = serde_json::from_str(input.as_str()).unwrap();

                        let program_input = cairo_input.program_input;
                        let program_input_w_driver_details =
                            format!("0x1,{},{}", driver.program_address, program_input);

                        // TODO: change error type of `perform_bind` to allow Cairo and WASM errors
                        self.provable_runtime
                            .execute_program(
                                self.ctx.user_id.clone(),
                                program_input_w_driver_details,
                                cairo_input.user_signature,
                            )
                            .await
                            .unwrap()
                    }
                };

                let path_info = PathInfo {
                    driver_name: driver_info.name,
                    driver_version: driver_info.version,
                    account_info: output,
                };

                self.driver_runtime
                    .resolver
                    .insert(path.clone(), path_info)
                    .await;
            }
            Some(existing) => {
                if existing.driver_name != driver_info.name
                    || existing.driver_version != driver_info.version
                {
                    return Err(
                        component::module::component::units::driver::DriverError::InvalidInput(
                            "Invalid driver binding".to_string(),
                        ),
                    );
                }

                let driver = self
                    .get_driver(&driver_info, self.driver_runtime.engine.clone())
                    .await?;
                let (mut linker, mut state) = self.get_lower_runtime(driver_info.clone())?;

                wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|err| {
                    component::module::component::units::driver::DriverError::SystemError(
                        err.to_string(),
                    )
                })?;

                let output = match driver {
                    ProgramComponent::WASM(driver) => {
                        let instance = component::driver::DriverWorld::instantiate_async(
                            &mut state,
                            &driver.module,
                            &linker,
                        )
                        .await
                        .map_err(|e| {
                            component::module::component::units::driver::DriverError::SystemError(
                                e.to_string(),
                            )
                        })?;

                        instance
                            .component_units_driver()
                            .call_bind(state, &input, Some(&existing.account_info))
                            .await
                            .map_err(|_| {
                                component::module::component::units::driver::DriverError::SystemError(
                                    "Failed while calling bind".to_string(),
                                )
                            })?
                            .map_err(|_| {
                                component::module::component::units::driver::DriverError::SystemError(
                                    "Failed while calling bind".to_string(),
                                )
                            })?
                    }
                    ProgramComponent::Cairo(driver) => {
                        // TODO: change error type of `perform_bind` to allow Cairo and WASM errors
                        let cairo_input: CairoInput = serde_json::from_str(input.as_str()).unwrap();

                        let program_input = cairo_input.program_input;
                        let program_input_w_driver_details =
                            format!("0x1,{},{}", driver.program_address, program_input);

                        // TODO: change error type of `perform_bind` to allow Cairo and WASM errors
                        // implement bind in Cairo
                        self.provable_runtime
                            .execute_program(
                                self.ctx.user_id.clone(),
                                program_input_w_driver_details,
                                cairo_input.user_signature,
                            )
                            .await
                            .unwrap()
                    }
                };

                let path_info = PathInfo {
                    driver_name: driver_info.name,
                    driver_version: driver_info.version,
                    account_info: output,
                };

                self.driver_runtime
                    .resolver
                    .insert(path.clone(), path_info)
                    .await;
            }
        }
        Ok(())
    }
}

impl DriverState {
    pub fn new(
        ctx: UserCtx,
        driver_ctx: DriverCtx,
        platform: Platform,
        event_sender: Arc<mpsc::Sender<Event>>,
    ) -> Self {
        Self {
            ctx,
            driver_ctx,
            platform,
            event_sender,
            // table: ResourceTable::new(),
            // wasi_ctx: wasmtime_wasi::WasiCtxBuilder::new().build(),
        }
    }
}

impl wasmtime_wasi::WasiView for DriverState {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        let resource = Box::new(wasmtime_wasi::ResourceTable::new());

        Box::leak(resource)
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        let ctx = Box::new(wasmtime_wasi::WasiCtx::builder().build());
        Box::leak(ctx)
    }
}

impl wasmtime_wasi::WasiView for ProcessState {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        let resource = Box::new(wasmtime_wasi::ResourceTable::new());

        Box::leak(resource)
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        let ctx = Box::new(wasmtime_wasi::WasiCtx::builder().build());
        Box::leak(ctx)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ProgramComponent {
    WASM(WasmComponent),
    Cairo(CairoComponent),
}

#[derive(Clone)]
pub struct WasmComponent {
    pub module: Component,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CairoComponent {
    pub program_address: String,
}
impl Serialize for WasmComponent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        // TODO: handle unwrap
        map.serialize_entry("module", self.module.serialize().unwrap().as_slice());
        map.end()
    }
}

// TODO: need to test this
impl<'de> Deserialize<'de> for WasmComponent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = WasmComponent;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "the parameters for `WasmComponent`")
            }

            #[allow(unused_mut)]
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // TODO: figure out a way to pass this form runtime
                let engine =
                    wasmtime::Engine::new(wasmtime::Config::new().async_support(true)).unwrap();
                let module: Component = unsafe {
                    Component::deserialize(
                        &engine,
                        seq.next_element::<Vec<u8>>()?.ok_or_else(|| {
                            serde::de::Error::invalid_length(1, &"expected 1 parameters")
                        })?,
                    )
                    .unwrap()
                };

                if seq.next_element::<serde::de::IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::invalid_length(
                        2,
                        &"expected 1 parameters",
                    ));
                }

                Ok(WasmComponent { module })
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::marker::PhantomData;

    fn check_send<T>(ty: PhantomData<T>)
    where
        T: Send,
    {
    }

    fn check_sync<T>(ty: PhantomData<T>)
    where
        T: Sync,
    {
    }

    /*
     */
    #[test]
    fn test_send() {
        check_sync::<DriverState>(PhantomData);
    }
}

use std::collections::HashMap;

use wasi_common::sync::WasiCtxBuilder;
use wasi_common::WasiCtx;

use crate::types::WasmString;

use super::driver::DriverRuntime;
use super::platform::Platform;

pub trait Binding<T> {
    fn bind(self, linker: &mut wasmtime::Linker<T>) -> anyhow::Result<()>;
}

impl Binding<State> for (DriverRuntime, Platform) {
    ///
    /// binding driver to linker
    /// This function will link functions that are exposed by the driver to the linker.
    /// These are `syscall` equivalent, presented to the WebAssembly module.
    ///
    /// This functions includes:
    /// - `intend`: Creating an intent, which is a builder pattern over time, to perform a
    /// - `transfer`: This performs transfer between 2 different active accounts.
    ///   transaction.
    /// - `done`: This function is called to finalize the intent and perform the transaction.
    ///
    fn bind(self, linker: &mut wasmtime::Linker<State>) -> anyhow::Result<()> {
        let driver = self.0.clone();
        let platform = self.1.clone();

        let intend = move |mut caller: wasmtime::Caller<'_, State>,
                           driver_name_ptr: i32,
                           driver_name_len: i32,
                           account_info_ptr: i32,
                           account_info_len: i32| {
            let driver = driver.clone();
            let platform = platform.clone();

            let driver_name =
                WasmString::from_caller(&mut caller, (driver_name_ptr, driver_name_len))?;
            let account_info =
                WasmString::from_caller(&mut caller, (account_info_ptr, account_info_len))?;

            let mut lower_store = wasmtime::Store::new(&driver.engine, ());
            let mut lower_linker = wasmtime::Linker::new(&driver.engine);
            platform.bind(&mut lower_linker)?;

            let driver_module = driver
                .drivers
                .read()
                .map_err(|_| anyhow::anyhow!("lock failed"))?;
            let driver_module = driver_module
                .get(driver_name.clone().into_str())
                .ok_or_else(|| anyhow::anyhow!("Driver not found"))?;

            let lower_instance = lower_linker.instantiate(&mut lower_store, driver_module)?;
            let lower_memory = lower_instance
                .get_memory(&mut lower_store, "memory")
                .ok_or_else(|| anyhow::anyhow!("No memory"))?;

            let input = account_info;
            let loaded_input = input.allocate_on_wasm(&lower_memory, &mut lower_store)?;
            let intend_caller = lower_instance
                .get_typed_func::<(i32, i32), (i32, i32)>(&mut lower_store, "intend")?;

            let result = intend_caller.call(&mut lower_store, loaded_input)?;
            let output = WasmString::from_memory(&lower_memory, &lower_store, result)?;

            // store output in the descriptor
            let key = crate::utils::id::new();
            caller.data_mut().descriptors.insert(
                key.clone(),
                Descriptor {
                    driver_name: driver_name.into_str().to_string(),
                    account_info: serde_json::from_str(output.into_str())?,
                },
            );

            let memory = caller
                .get_export("memory")
                .and_then(|m| m.into_memory())
                .ok_or_else(|| anyhow::anyhow!("No memory"))?;

            let loaded_str = WasmString::new(&key).allocate_on_caller(&memory, &mut caller)?;

            Ok(loaded_str)
        };

        linker.func_wrap("driver", "intend", intend)?;

        let driver = self.0.clone();
        let platform = self.1.clone();

        let done = move |mut caller: wasmtime::Caller<'_, State>, key_ptr: i32, key_len: i32| {
            let driver = driver.clone();
            let platform = platform.clone();
            let key = WasmString::from_caller(&mut caller, (key_ptr, key_len))?;
            let value = caller.data().descriptors.get(key.clone().into_str());

            match value {
                None => anyhow::bail!("Descriptor not found"),
                Some(value) => {
                    // get the driver, and execute the done function

                    let driver_name = value.driver_name.clone();
                    let account_info = value.account_info.clone();

                    let mut lower_store = wasmtime::Store::new(&driver.engine, ());
                    let mut lower_linker = wasmtime::Linker::new(&driver.engine);
                    platform.bind(&mut lower_linker)?;

                    let driver_module = driver
                        .drivers
                        .read()
                        .map_err(|_| anyhow::anyhow!("lock failed"))?;

                    let driver_module = driver_module
                        .get(&driver_name)
                        .ok_or_else(|| anyhow::anyhow!("Driver not found"))?;

                    let lower_instance =
                        lower_linker.instantiate(&mut lower_store, driver_module)?;

                    let lower_memory = lower_instance
                        .get_memory(&mut lower_store, "memory")
                        .ok_or_else(|| anyhow::anyhow!("No memory"))?;

                    let input = serde_json::to_string(&account_info)?;

                    let loaded_input = WasmString::new(&input)
                        .allocate_on_wasm(&lower_memory, &mut lower_store)?;

                    let done_caller = lower_instance
                        .get_typed_func::<(i32, i32), ()>(&mut lower_store, "done")?;

                    done_caller.call(&mut lower_store, loaded_input)?;

                    caller.data_mut().descriptors.remove(key.into_str());
                }
            }

            Ok(())
        };

        linker.func_wrap("driver", "done", done)?;

        Ok(())
    }
}

impl Binding<()> for Platform {
    fn bind(self, linker: &mut wasmtime::Linker<()>) -> anyhow::Result<()> {
        let storage = self.storage.clone();
        linker.func_wrap(
            "platform",
            "get",
            move |mut caller: wasmtime::Caller<'_, ()>, key_ptr: i32, key_len: i32| {
                let key = WasmString::from_caller(&mut caller, (key_ptr, key_len))?;
                let output = storage.get(key.into_str())?;
                match output {
                    Some(value) => {
                        let value = WasmString::new(&value);
                        let memory = caller
                            .get_export("memory")
                            .and_then(|m| m.into_memory())
                            .ok_or_else(|| anyhow::anyhow!("No memory"))?;

                        let loaded_str = value.allocate_on_caller(&memory, &mut caller)?;

                        Ok(loaded_str)
                    }
                    None => Ok((0, 0)),
                }
            },
        )?;

        let storage = self.storage.clone();

        linker.func_wrap(
            "platform",
            "set",
            move |mut caller: wasmtime::Caller<'_, ()>,
                  key_ptr: i32,
                  key_len: i32,
                  value_ptr: i32,
                  value_len: i32| {
                let key = WasmString::from_caller(&mut caller, (key_ptr, key_len))?;
                let value = WasmString::from_caller(&mut caller, (value_ptr, value_len))?;

                storage.set(key.into_str(), value.into_str())?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

pub struct State {
    pub resolver: super::resolver::Resolver,
    pub descriptors: HashMap<String, Descriptor>,
    pub wasi: WasiCtx,
}

pub struct Descriptor {
    driver_name: String,
    account_info: serde_json::Value,
}

impl State {
    pub fn new(resolver: super::resolver::Resolver) -> Self {
        Self {
            resolver,
            descriptors: HashMap::new(),
            wasi: WasiCtxBuilder::new().build(),
        }
    }
}

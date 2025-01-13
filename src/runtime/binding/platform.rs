use std::sync::mpsc;

use crate::runtime::platform::Platform;
use crate::runtime::types::Event;
use crate::types::WasmString;

use super::driver::PlatformState;
use super::Binding;

impl Binding<PlatformState> for Platform {
    fn bind(
        self,
        linker: &mut wasmtime::Linker<PlatformState>,
        event_bridge: mpsc::Sender<Event>,
    ) -> anyhow::Result<()> {
        let storage = self.storage.clone();
        let event_channel = event_bridge.clone();

        linker.func_wrap(
            "platform",
            "get",
            move |mut caller: wasmtime::Caller<'_, PlatformState>, key_ptr: i32, key_len: i32| {
                tracing::info!(system = "platform", func = "get", "syscall");
                let key = WasmString::from_caller(&mut caller, (key_ptr, key_len))?;

                event_channel.send(Event {
                    loc: crate::runtime::types::Loc::Start,
                    event_type: crate::runtime::types::EventType::Info,
                    level: crate::runtime::types::Level::Platform,
                    call_type: crate::runtime::types::CallType::Get,
                    data: serde_json::json!({
                        "key": key.clone().into_str(),
                    }),
                })?;

                tracing::info!(key = ?key, "Getting key");

                let output = storage.get(key.clone().into_str())?;
                match output {
                    Some(value) => {
                        let value = WasmString::new(&value);
                        let memory = caller
                            .get_export("memory")
                            .and_then(|m| m.into_memory())
                            .ok_or_else(|| anyhow::anyhow!("No memory"))?;

                        let loaded_str = value.allocate_on_caller(&memory, &mut caller)?;

                        event_channel.send(Event {
                            loc: crate::runtime::types::Loc::End,
                            event_type: crate::runtime::types::EventType::Info,
                            level: crate::runtime::types::Level::Platform,
                            call_type: crate::runtime::types::CallType::Get,
                            data: serde_json::json!({
                                "key": key.into_str(),
                                "value": value.into_str(),
                            }),
                        })?;

                        Ok(loaded_str)
                    }
                    None => {
                        event_channel.send(Event {
                            loc: crate::runtime::types::Loc::End,
                            event_type: crate::runtime::types::EventType::Error,
                            level: crate::runtime::types::Level::Platform,
                            call_type: crate::runtime::types::CallType::Get,
                            data: serde_json::json!({
                                "key": key.into_str(),
                                "error": "Key not found",
                            }),
                        })?;

                        Ok((0, 0))
                    }
                }
            },
        )?;

        let storage = self.storage.clone();
        let event_channel = event_bridge.clone();

        linker.func_wrap(
            "platform",
            "set",
            move |mut caller: wasmtime::Caller<'_, PlatformState>,
                  key_ptr: i32,
                  key_len: i32,
                  value_ptr: i32,
                  value_len: i32| {
                tracing::info!(system = "platform", func = "set", "syscall");
                let key = WasmString::from_caller(&mut caller, (key_ptr, key_len))?;
                let value = WasmString::from_caller(&mut caller, (value_ptr, value_len))?;

                event_channel.send(Event {
                    loc: crate::runtime::types::Loc::Start,
                    event_type: crate::runtime::types::EventType::Info,
                    level: crate::runtime::types::Level::Platform,
                    call_type: crate::runtime::types::CallType::Set,
                    data: serde_json::json!({
                        "key": key.clone().into_str(),
                        "value": value.clone().into_str(),
                    }),
                })?;

                storage.set(key.clone().into_str(), value.clone().into_str())?;

                event_channel.send(Event {
                    loc: crate::runtime::types::Loc::End,
                    event_type: crate::runtime::types::EventType::Info,
                    level: crate::runtime::types::Level::Platform,
                    call_type: crate::runtime::types::CallType::Set,
                    data: serde_json::json!({
                        "key": key.into_str(),
                        "value": value.into_str(),
                    }),
                })?;

                Ok(())
            },
        )?;

        Ok(())
    }
}

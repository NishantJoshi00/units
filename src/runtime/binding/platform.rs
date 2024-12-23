use crate::runtime::platform::Platform;
use crate::types::WasmString;

use super::Binding;

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

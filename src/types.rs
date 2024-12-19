use anyhow::Context;

pub struct WasmString<'a>(&'a str);

impl<'a> WasmString<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn into_str(self) -> &'a str {
        self.0
    }

    pub fn allocate_on_wasm<T>(
        &'a self,
        memory: &wasmtime::Memory,
        store: &mut wasmtime::Store<T>,
    ) -> anyhow::Result<(i32, i32)> {
        let ptr = memory.data_size(&store);
        let bytes = self.0.as_bytes();

        let delta = (bytes.len() + ptr) / 65536 + 1;

        memory.grow(&mut *store, delta.try_into()?)?;
        memory.write(&mut *store, ptr, bytes)?;

        Ok((ptr as i32, bytes.len() as i32))
    }

    pub fn from_caller<T>(
        mut caller: wasmtime::Caller<'_, T>,
        (ptr, len): (i32, i32),
    ) -> anyhow::Result<Self> {
        let memory = caller
            .get_export("memory")
            .and_then(wasmtime::Extern::into_memory)
            .ok_or_else(|| anyhow::anyhow!("Failed to get memory export"))?;

        let mut buffer = vec![0u8; len as usize];

        memory
            .read(&mut caller, ptr as usize, &mut buffer)
            .context("Failed to read from runtime")?;

        let output = std::str::from_utf8(&buffer)
            .context("Failed to interpret it as string")?
            .to_string();

        let leaked_string = Box::leak(output.into_boxed_str());

        Ok(Self(leaked_string))
    }
}

use std::collections::HashMap;

mod driver;
mod platform;

pub trait Binding<T> {
    fn bind(self, linker: &mut wasmtime::Linker<T>) -> anyhow::Result<()>;
}

pub struct State {
    pub resolver: super::resolver::Resolver,
    pub descriptors: HashMap<String, Descriptor>,
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
        }
    }
}

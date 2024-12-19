pub trait Binding {
    fn bind(&self, linker: &mut wasmtime::Linker<()>) -> anyhow::Result<()>;
}

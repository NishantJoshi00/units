/// [`Dispatcher`] is a state machine that manages the lifecycle of a Wasm module.
///
/// This is used to load and execute Wasm modules. The dispatcher can be used in 2 places.
/// 1. This can be used in the process layer to load and execute programs that are submitted by the
///    users.
/// 2. From the programs, when the system types to invoke driver functions, they can be accessed
///    from the driver specific dispatcher.
pub enum Dispatcher<S: Bindings> {
    PreLoad {
        context: S,
        module: wasmtime::Module,
        linker: wasmtime::Linker<S>,
    },
    Executable {
        instance: wasmtime::Instance,
    },
}

pub trait Bindings: Sized {
    fn bind(&self, linker: &mut wasmtime::Linker<Self>);
}

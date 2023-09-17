use std::sync::Mutex;

use wasmtime::*;
use wasmtime_wasi::*;
pub struct AppState {
    pub linker: Mutex<Linker<WasiCtx>>,
    pub module: Mutex<Module>,
    pub store: Mutex<Store<WasiCtx>>
}

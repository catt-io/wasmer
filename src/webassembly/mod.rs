pub mod errors;
pub mod instance;
pub mod memory;
pub mod module;
pub mod relocation;
pub mod utils;

use cranelift_native;
use std::panic;
use std::ptr;
use std::str::FromStr;
use std::time::{Duration, Instant};
use target_lexicon::{self, Triple};
use wasmparser;

pub use self::errors::{Error, ErrorKind};
pub use self::instance::Instance;
pub use self::memory::LinearMemory;
pub use self::module::Module;

pub struct ResultObject {
    /// A webassembly::Module object representing the compiled WebAssembly module.
    /// This Module can be instantiated again
    pub module: Module,
    /// A webassembly::Instance object that contains all the Exported WebAssembly
    /// functions.
    pub instance: Instance,
}

pub struct ImportObject {}

/// The webassembly::instantiate() function allows you to compile and
/// instantiate WebAssembly code
/// Params:
/// * `buffer_source`: A `Vec<u8>` containing the
///   binary code of the .wasm module you want to compile.
/// * `import_object`: An object containing the values to be imported
///   into the newly-created Instance, such as functions or
///   webassembly::Memory objects. There must be one matching property
///   for each declared import of the compiled module or else a
///   webassembly::LinkError is thrown.
/// Errors:
/// If the operation fails, the Result rejects with a
/// webassembly::CompileError, webassembly::LinkError, or
/// webassembly::RuntimeError, depending on the cause of the failure.
pub fn instantiate(
    buffer_source: Vec<u8>,
    import_object: Option<ImportObject>,
) -> Result<ResultObject, ErrorKind> {
    let module = compile(buffer_source)?;
    debug!("webassembly - creating instance");
    let instance = Instance::new(&module, ptr::null())?;
    debug!("webassembly - instance created");
    Ok(ResultObject { module, instance })
}

/// The webassembly::compile() function compiles a webassembly::Module
/// from WebAssembly binary code.  This function is useful if it
/// is necessary to a compile a module before it can be instantiated
/// (otherwise, the webassembly::instantiate() function should be used).
/// Params:
/// * `buffer_source`: A `Vec<u8>` containing the
///   binary code of the .wasm module you want to compile.
/// Errors:
/// If the operation fails, the Result rejects with a
/// webassembly::CompileError.
pub fn compile(buffer_source: Vec<u8>) -> Result<Module, ErrorKind> {
    // TODO: This should be automatically validated when creating the Module
    let valid = validate(&buffer_source);
    debug!("webassembly - valid {:?}", valid);
    if !valid {
        return Err(ErrorKind::CompileError("Module not valid".to_string()));
    }

    debug!("webassembly - creating module");
    let module = Module::from_bytes(buffer_source, triple!("riscv64"), None)?;
    debug!("webassembly - module created");

    Ok(module)
}

/// The webassembly::validate() function validates a given typed
/// array of WebAssembly binary code, returning whether the bytes
/// form a valid wasm module (true) or not (false).
/// Params:
/// * `buffer_source`: A `Vec<u8>` containing the
///   binary code of the .wasm module you want to compile.
pub fn validate(buffer_source: &Vec<u8>) -> bool {
    wasmparser::validate(buffer_source, None)
}
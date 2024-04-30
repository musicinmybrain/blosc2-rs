#![allow(
    non_upper_case_globals,
    unused_imports,
    non_snake_case,
    improper_ctypes,
    non_camel_case_types
)]

pub use libc;
use libc::{c_char, timespec, FILE};

#[cfg(not(feature = "regenerate-bindings"))]
mod bindings;
#[cfg(not(feature = "regenerate-bindings"))]
pub use bindings::*;

extern "C" {
    fn ruapu_init();
    fn ruapu_supports(isa: *const c_char) -> i32;
}

#[no_mangle]
pub extern "C" fn __builtin_cpu_supports(n: *const c_char) -> bool {
    unsafe {
        ruapu_init();
        ruapu_supports(n) != 0
    }
}

#[cfg(feature = "regenerate-bindings")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

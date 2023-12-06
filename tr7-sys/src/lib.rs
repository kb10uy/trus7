#![allow(non_camel_case_types)]

use libc::{c_uint, c_void, size_t};

#[link(name = "libtr7")]
extern "C" {
    pub fn tr7_engine_create(config: *const tr7_config_t) -> tr7_engine_t;
    pub fn tr7_engine_destroy(engine: tr7_engine_t);
}

pub type tr7_engine_t = *mut c_void;

#[repr(C)]
pub struct tr7_config_t {
    dictionary_size: c_uint,
    malloc: extern "C" fn(size_t) -> *mut c_void,
    free: extern "C" fn(*mut c_void),
}

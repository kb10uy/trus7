use std::ptr::null;

use tr7_sys::{tr7_engine_create, tr7_engine_destroy, tr7_engine_t};

pub struct Engine {
    engine: tr7_engine_t,
}

impl Engine {
    pub fn new() -> Engine {
        let engine = unsafe { tr7_engine_create(null()) };
        if engine.is_null() {
            panic!("failed to initialize engine");
        }

        Engine { engine }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            tr7_engine_destroy(self.engine);
        }
    }
}

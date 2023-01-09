mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static mut TEST: u32 = 0;

#[wasm_bindgen]
pub fn increment() -> u32 {
    unsafe {
        TEST += 1;
        TEST
    }
}

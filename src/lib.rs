#![allow(dead_code)]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate rand;

mod utils;
mod piece;
mod canvas;

use cfg_if::cfg_if;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}



//#[wasm_bindgen]
//extern {
//    fn alert(s: &str);
//}
//
//#[wasm_bindgen]
//pub fn greet(s: &str) {
//    alert(&format!("Hello, {}!", s));
//}

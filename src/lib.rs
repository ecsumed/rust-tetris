#![allow(dead_code)]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate rand;
extern crate web_sys;

mod utils;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


mod piece;
mod canvas;

//#[wasm_bindgen]
//extern {
//    fn alert(s: &str);
//}
//
//#[wasm_bindgen]
//pub fn greet(s: &str) {
//    alert(&format!("Hello, {}!", s));
//}

#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "./defined-in-js")]
extern "C" {
    fn update(s: &str);
}

#[wasm_bindgen]
pub fn text_replace(from: &str, to: &str, text: &str) {
    let stext = text.to_string();
    let newtext = stext.replace(from,to);
    update(&newtext);
}
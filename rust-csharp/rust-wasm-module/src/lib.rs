use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[wasm_bindgen]
pub fn get_len_of_str(text: &str) -> u32 {
    text.chars().count() as u32
}

#[wasm_bindgen]
pub fn turn_to_upper(text: &str) -> String {
    text.to_uppercase()
}

use wasm_bindgen::prelude::*;
use js_sys::Math::*;
use conv::*;

#[wasm_bindgen]
pub fn random_name(x: Box<[JsValue]>) -> Result<JsValue, JsValue> {
    for name in x.iter() {
        if !name.is_string() {
            return Err("Array contains elements that are no strings.".into());
        }
    }    

    return Result::Ok(x[(random() * f64::value_from(x.len()).unwrap()).floor() as usize].clone())
}

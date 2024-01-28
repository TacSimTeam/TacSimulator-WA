use wasm_bindgen::prelude::{wasm_bindgen, Closure};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setTimeout")]
    pub fn set_timeout(handler: &Closure<dyn FnMut()>, timeout: i32) -> i32;

    #[wasm_bindgen(js_name = "clearTimeout")]
    pub fn clear_timeout(handle: i32);
}

pub struct Timeout {
    pub id: i32,
}

impl Timeout {
    pub fn new(closure: Closure<dyn FnMut()>, mills: u32) -> Self {
        let id = set_timeout(&closure, mills as i32);
        closure.forget();
        Self { id }
    }
}

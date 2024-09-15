use leptos::wasm_bindgen;
use leptos::wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/local_storage.js")]
extern "C" {
    pub fn setItem(key: &str, value: &str) -> JsValue;
    pub fn getItem(key: &str) -> JsValue;
    pub fn removeItem(key: &str);
}

macro_rules! import{
  ($($module:ident), +) => {
      $(
        mod $module;
        #[allow(unused_imports)]
        pub use $module::*;
    )*
  };
}

pub(crate) use import;

use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn load_plugins(name: &str) {
    alert(&format!("Hello, {}!", name));
    if let Ok(data) = reqwest::blocking::get("https://www.rust-lang.org"){
        alert(data.text().unwrap().as_str());
    }else{

    };

}

use wasm_bindgen::prelude::*;

// JavaScript로 내보낼 함수
#[wasm_bindgen]
pub fn gugudan(n: i32) -> String {
    let mut result = String::new();
    for i in 1..=9 {
        result.push_str(&format!("{} * {} = {}\n", n, i, n * i));
    }
    result
}

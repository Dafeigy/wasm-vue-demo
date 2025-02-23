use wasm_bindgen::prelude::*; 

#[wasm_bindgen]
pub fn add(left: f32, right: f32) -> f32 {
    return left + right;
}
#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    if n <= 0 {
        -1
    } else if (n == 1) || (n == 2) {
        1
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

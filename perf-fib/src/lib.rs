use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "fibonacciMany")]
pub fn fibonacci_many(repeat: i32, start: i32, end: i32, use_memory: bool) -> i32 {
    let mut results = Vec::<i32>::new();
    let mut count = 0;
    for _ in 0..repeat {
        for i in start..end {
            let f = fibonacci(i);
            count += 1;
            if use_memory {
                results.push(f);
            }
        }
    }

    count
}

#[wasm_bindgen]
pub fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a
}

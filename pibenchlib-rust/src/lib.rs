#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
  fn random() -> f64;
}

fn monte_carlo_pi(reps: u32) -> u64 {
    let mut count = 0;

    for _ in 0..reps {
        let x = random();
        let y = random();

        if in_unit_circle(x, y) {
            count += 1;
        }
    }

    return count;
}

fn in_unit_circle(x: f64, y: f64) -> bool {
    x * x + y * y < 1.0
}

#[wasm_bindgen]
pub fn approximate_pi(n: u32) -> f64 {
    let hits = monte_carlo_pi(n);
    hits as f64 / n as f64 * 4.0
}
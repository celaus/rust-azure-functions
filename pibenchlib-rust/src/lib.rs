#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate rand;
extern crate wasm_bindgen;

use rand::Rng;
use wasm_bindgen::prelude::*;


fn monte_carlo_pi(reps: u64) -> u64 {
    let mut count = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..reps {
        let x = rng.gen::<f64>();
        let y = rng.gen::<f64>();

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
pub fn approximate_pi(n: u64) -> f64 {
    let hits = monte_carlo_pi(n);
    hits as f64 / n as f64 * 4.0
}
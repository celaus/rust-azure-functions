#![feature(test, int_to_from_bytes, proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
extern crate test;
mod wh;
mod fib;

use fib::fib;

use wasm_bindgen::prelude::*;
use wh::WichmannHillRng;

fn monte_carlo_pi(reps: u32) -> u32 {
    let mut count = 0;
    let mut rnd = WichmannHillRng::seeded(0x5ff);
    for _ in 0..reps {
        let x = rnd.next_f32();
        let y = rnd.next_f32();
        if in_unit_circle(x, y) {
            count += 1;
        }
    }
    return count;
 }

fn in_unit_circle(x: f32, y: f32) -> bool {
    x * x + y * y < 1.0
}

#[wasm_bindgen]
pub fn approximate_pi(n: u32) -> f32 {
    let hits = monte_carlo_pi(n);
    hits as f32 / n as f32 * 4.0
}

#[wasm_bindgen]
pub fn run_fibonacci_test(n: u32) -> u32 {
    fib(n)
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[test]
    fn test_nothing() {
        // for x in 0..1000 {
        //     let hits = monte_carlo_pi(x);
        //     let y = hits as f64 / x as f64 * 4.0;
        //     println!("{}",y);
        // } 
        assert_eq!(1,1);
    }


    #[test]
    fn test_in_unit_circle_is_true() {
        assert_eq!(in_unit_circle(0.1, 0.1), true);
        assert_eq!(in_unit_circle(0.5, 0.5), true);
        assert_eq!(in_unit_circle(0.7, 0.7), true);
    }


    #[test]
    fn test_in_unit_circle_is_false() {
        assert_eq!(in_unit_circle(0.71, 0.71), false);
        assert_eq!(in_unit_circle(1.0, 1.0), false);
        assert_eq!(in_unit_circle(2.0, 1.0), false);
        assert_eq!(in_unit_circle(10.0, 10.0), false);
    }

    #[bench]
    fn bench_monte_carlo_pi(b: &mut Bencher) {
        b.iter(|| monte_carlo_pi(1000));
    }
}
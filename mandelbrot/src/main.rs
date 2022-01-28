extern crate num;
use num::Complex;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}
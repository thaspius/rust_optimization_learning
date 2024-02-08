use std::f64::consts::PI;
use num::complex::Complex;
fn main() {
    let x = Complex::new(0.0, 0.001);
    let y = Complex::new(PI/2.0, 0.0);
    let v = (y+x).powi(2).sin();

    println!("r: {}", v.re);
    println!("i: {}", v.im /0.001);
}

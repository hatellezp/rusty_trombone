use num::complex::Complex;
// use rustfft::FftPlanner;
use std::f32::consts::PI;
// use std::fs::File;
// use std::io::BufReader;

pub fn evaluate(point: f32, coeffs: &[f32], offset: Option<usize>) -> f32 {
    let offset = offset.unwrap_or(0);
    let period = coeffs.len() + offset;
    let f0 = (2. * PI) / (period as f32);

    (0..period)
        .map(|i| {
            if i < offset {
                0.
            } else {
                *coeffs.get(i - offset).unwrap()
            }
        })
        .enumerate()
        .fold(Complex::new(0., 0.), |accum, (index, value)| {
            let arg = Complex::new(point * (index as f32) * f0, 0.);
            let rho = Complex::new(value, 0.);
            accum + rho * arg.sin()
        })
        .re
}
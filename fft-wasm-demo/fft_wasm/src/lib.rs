use wasm_bindgen::prelude::*;
use std::f64::consts::PI;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

#[wasm_bindgen]
impl Complex {
    #[wasm_bindgen(constructor)]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl Complex {
    fn add(&self, other: &Complex) -> Complex {
        Complex { re: self.re + other.re, im: self.im + other.im }
    }
    fn sub(&self, other: &Complex) -> Complex {
        Complex { re: self.re - other.re, im: self.im - other.im }
    }
    fn mul(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
    fn from_polar(r: f64, theta: f64) -> Complex {
        Complex {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }
}

/// FFT (재귀)
#[wasm_bindgen]
pub fn fft_js(real: &[f64], imag: &[f64]) -> Vec<f64> {
    let mut data: Vec<Complex> = real.iter().zip(imag.iter()).map(|(&r, &i)| Complex::new(r, i)).collect();
    fft(&mut data);

    // 결과를 [re0, im0, re1, im1, ...] 형태로 반환
    let mut out = Vec::with_capacity(data.len() * 2);
    for x in data {
        out.push(x.re);
        out.push(x.im);
    }
    out
}

/// 내부 FFT 로직
fn fft(input: &mut [Complex]) {
    let n = input.len();
    if n <= 1 { return; }

    let mut even: Vec<Complex> = input.iter().step_by(2).copied().collect();
    let mut odd: Vec<Complex> = input.iter().skip(1).step_by(2).copied().collect();

    fft(&mut even);
    fft(&mut odd);

    for k in 0..n/2 {
        let twiddle = Complex::from_polar(1.0, -2.0 * PI * k as f64 / n as f64);
        let t = twiddle.mul(&odd[k]);
        input[k] = even[k].add(&t);
        input[k + n/2] = even[k].sub(&t);
    }
}

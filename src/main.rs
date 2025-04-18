//Rufft : A Radix-2 Fast Fourier Transform Implementation in Rust
//by pranav-avn

//TODO:
//Implement Butterfly Structure

use std::f64::consts::PI;
use num::complex::Complex;

fn bit_reverse(input: &mut [Complex<f64>]){
    let n = input.len();
    let bits = (n as f64).log2() as usize; //number of bits required to represent the input
    
    for i in 0..n{
        let rev = reverse_bit_order(i, bits);
        if i < rev{
            input.swap(i,rev);
        }
    }
}

fn reverse_bit_order(mut x: usize, bits: usize) -> usize{
    let mut result: usize = 0;
    for _ in 0..bits{
        result = (result << 1) | (x & 1); //shift LSB left
        x >>= 1; //move to next bit
    }
    result
}

fn precompute_twiddle(n:usize, invert:bool) -> Vec<Complex<f64>>{
    let sign: f64 = if invert {1.0} else {-1.0};
    (0..n/2).map(|k| Complex::from_polar(1.0, sign * 2.0 * PI * k as f64 / n as f64)).collect()
}

pub fn fft(mut data: &mut [Complex<f64>], invert:bool, twiddles: &[Complex<f64>]) {
    let n = data.len();
    assert!(n.is_power_of_two(), "Length must be a power of two");

    bit_reverse(&mut data);

    let mut len = 2;
    while len <= n{
        let step = n/len;

        for base in (0..n).step_by(len){
            for offset in 0..len/2{
                let a = data[base + offset];
                let b = data[base + offset + len/2] * twiddles[offset * step];
                data[base + offset] = a+b;
                data[base + offset + len/2] = a - b;
            }
        }
        len <<= 1;
    }

    if invert{
        let scale = 1.0/n as f64;
        for x in data.iter_mut(){
            *x *= scale;
        }
    }
}

fn main() {
    let mut data = vec![
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];

    let n: usize = data.len();
    let twiddles: Vec<Complex<f64>> = precompute_twiddle(n, false);
    fft(&mut data, false, &twiddles);

    println!("FFT result:");
    for (i, x) in data.iter().enumerate() {
        println!("  [{}] = {}", i, x);
    }

}

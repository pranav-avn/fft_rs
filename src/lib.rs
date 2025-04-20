/// fft_rs: A Radix-2 Fast Fourier Transform Implementation in Rust
///
/// This library provides an implementation of the Radix-2 FFT algorithm. 
/// It operates on a mutable reference to a vector of `Complex<f64>` values, 
/// where the size of the vector must be a power of 2.
///
/// # Key Features:
/// - **Bit Reversal**: The input vector's bit order is reversed using the `bit_reverse` function.
/// - **Twiddle Factor Computation**: Twiddle factors are precomputed for the input vector of size `n`.
/// - **Butterfly Structure**: The butterfly structure is implemented, and the resulting stage's outputs 
///   are stored back into the input vector.
///
/// # Usage:
/// - Pass a mutable reference to a vector of `Complex<f64>` values to the `fft` function.
/// - Ensure the size of the vector is a power of 2.
/// - Specify whether to perform a forward FFT (`invert = false`) or an inverse FFT (`invert = true`).
///
/// # Example:
/// ```rust
/// use num::complex::Complex;
/// use fft_rs::fft;
///
/// let mut data = vec![
///     Complex::new(0.0, 0.0),
///     Complex::new(1.0, 0.0),
///     Complex::new(0.0, 0.0),
///     Complex::new(-1.0, 0.0),
/// ];
///
/// fft(&mut data, false); // Perform forward FFT
/// ```
///
/// # Notes:
/// - The input vector is modified in place.
/// - For inverse FFT, the result is scaled by the size of the input vector.



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

pub fn fft(mut data: &mut [Complex<f64>], invert:bool) {
    let n = data.len();
    assert!(n.is_power_of_two(), "Length must be a power of two");

    bit_reverse(&mut data);
    let twiddles: Vec<Complex<f64>> = precompute_twiddle(n, invert);

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
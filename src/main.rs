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

fn main() {
    let mut data = vec![
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
        Complex::new(4.0, 0.0),
        Complex::new(5.0, 0.0),
        Complex::new(6.0, 0.0),
        Complex::new(7.0, 0.0),
    ];

    let n: usize = data.len();
    let twiddles: Vec<Complex<f64>> = precompute_twiddle(n, false);


    println!("Bit Reversed Input:");
    bit_reverse(&mut data);

    for (i, val) in data.iter().enumerate(){
        println!("Index {}: {}",i,val);
    }

    println!("\nTwiddle Factors");
    for (i, val) in twiddles.iter().enumerate(){
        println!("Index {}: {}",i,val);
    }

}

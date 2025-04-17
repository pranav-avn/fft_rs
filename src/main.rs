//Rufft : A Radix-2 Fast Fourier Transform Implementation in Rust
//by pranav-avn

//TODO:
//Implement Butterfly Structure
//Implement Twiddle Factor Calculation

//use std::f64::consts::PI;
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

    println!("Bit Reversed Input:");
    bit_reverse(&mut data);

    for (i, val) in data.iter().enumerate(){
        println!("Index {}: {}",i,val);
    }

}

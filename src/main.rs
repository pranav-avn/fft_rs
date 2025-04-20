//Rufft : A Radix-2 Fast Fourier Transform Implementation in Rust
//by pranav-avn

use num::complex::Complex;
use rufft::fft;

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

    fft(&mut data, false);

    println!("FFT result:");
    for (i, x) in data.iter().enumerate() {
        println!("  [{}] = {:.3}", i, x);
    }

    fft(&mut data, true);

    println!("Inverse FFT result:");
    for (i, x) in data.iter().enumerate() {
        println!("  [{}] = {:.3}", i, x);
    }

}

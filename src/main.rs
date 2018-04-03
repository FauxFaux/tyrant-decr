#[macro_use]
extern crate error_chain;
extern crate itertools;
extern crate rustfft;

use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem;

use itertools::Itertools;
use rustfft::FFT;
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex32;
use rustfft::num_traits::Zero;

mod errors;

use errors::*;

quick_main!(run);

const FFT_SIZE: usize = 64;
const FFT_SIZE_BYTES: usize = FFT_SIZE * 2;

const RADIO_BUF_SIZE: usize = 16 * 16384;
const RADIO_SAMPLE_RATE: usize = 2048 * 1000;

fn run() -> Result<()> {
    let fft = Radix4::new(FFT_SIZE, false);

    let mut highest = [0u64; FFT_SIZE];

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    loop {
        let mut buf_cu8 = [0u8; RADIO_BUF_SIZE];
        stdin.read_exact(&mut buf_cu8).unwrap();

        for block in 0..RADIO_BUF_SIZE / FFT_SIZE_BYTES {
            let mut input = [Complex32::zero(); FFT_SIZE];
            let sub = &buf_cu8[block * FFT_SIZE_BYTES..];
            let sub = &sub[..FFT_SIZE_BYTES];

            for i in 0..FFT_SIZE {
                input[i].re = to_float(sub[i * 2]);
                input[i].im = to_float(sub[i * 2 + 1]);
            }

            let mut output = [Complex32::zero(); FFT_SIZE];
            fft.process(&mut input, &mut output);

            output[0] = Complex32::zero(); // dc bias lol
            highest[output
                        .into_iter()
                        .enumerate()
                        .max_by(|&(_, left), &(_, right)| left.norm_sqr().partial_cmp(&right.norm_sqr()).unwrap())
                        .unwrap()
                        .0] += 1;
        }

        println!("{:?}", highest.iter().map(|x| x/1000).collect_vec());
    }
}

fn to_float(val: u8) -> f32 {
    f32::from(val) / 256.
}

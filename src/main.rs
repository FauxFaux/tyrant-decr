#[macro_use]
extern crate error_chain;
extern crate itertools;
extern crate rustfft;
extern crate sdr;

use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::mem;

use itertools::Itertools;
use rustfft::num_complex::Complex32;
use rustfft::num_traits::Zero;
use rustfft::FFT;
use rustfft::algorithm::Radix4;

mod errors;

use errors::*;

quick_main!(run);

const DECIMATE: usize = 1;
const FFT_SIZE: usize = 32;

const RADIO_BUF_SIZE: usize = 16 * 16384;
const RADIO_SAMPLE_RATE: usize = 2048 * 1000;

fn run() -> Result<()> {
    // todo: what even filter is this?
//    let mut fir = sdr::FIR::cic_compensator(128, 5, 7, DECIMATE);
    let mut fir = sdr::FIR::new(&tappity(&TAPS), DECIMATE, 1);
    let fft = Radix4::new(FFT_SIZE, false);

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(format!("pd.sr{}.cs8", RADIO_SAMPLE_RATE / DECIMATE))?;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    loop {
        let mut buf = [0u8; RADIO_BUF_SIZE];
        stdin.read_exact(&mut buf)?;

        // todo: shift

        let decimated = fir.process(unsafe { mem::transmute::<&[u8], &[i8]>(&buf) });

        assert_eq!(decimated.len() * DECIMATE, RADIO_BUF_SIZE);
        f.write_all(unsafe { mem::transmute::<&[i8], &[u8]>(&decimated) })?;

        let mut floats: Vec<Complex32> = decimated
            .into_iter()
            .tuples()
            .map(|(i, q)| Complex32::new(to_float(i), to_float(q)))
            .collect();

        for block in 0..floats.len() / FFT_SIZE {
            let mut output = [Complex32::zero(); FFT_SIZE];
            let start = block * FFT_SIZE;
            fft.process(&mut floats[start..start + FFT_SIZE], &mut output);
            println!(
                "{} {}",
                real_sum(&output[2..FFT_SIZE / 2]),
                real_sum(&output[FFT_SIZE / 2..output.len() - 2])
            );
        }
    }
}

fn real_sum(complexes: &[Complex32]) -> f32 {
    complexes.into_iter().map(|&c| c.norm_sqr()).sum()
}

fn to_float(val: i8) -> f32 {
    f32::from(i16::from(val) + 128) / 255.
}

// ???
fn to_i16(val: f32) -> i16 {
    ((val) * 2.) as i16
}

fn tappity(taps: &[f32]) -> Vec<i16> {
    taps.iter().map(|&f| to_i16(f)).collect()
}

// from gnuradio.filter import firdes
// firdes.low_pass(gain=32/2, sampling_freq=2048*1000, cutoff_freq=50*1000, transition_width=50*1000)
const TAPS: [f32; 99] = [
    0.007827633991837502,
    0.00755667919293046,
    0.007241279352456331,
    0.006825620774179697,
    0.006233078893274069,
    0.005370331462472677,
    0.004133045207709074,
    0.002412923611700535,
    0.00010584338451735675,
    -0.0028792729135602713,
    -0.006611250340938568,
    -0.011127560399472713,
    -0.016425788402557373,
    -0.022456131875514984,
    -0.02911534160375595,
    -0.03624257445335388,
    -0.043617404997348785,
    -0.050960298627614975,
    -0.05793576315045357,
    -0.064158134162426,
    -0.0692000463604927,
    -0.07260339707136154,
    -0.0738925039768219,
    -0.07258912175893784,
    -0.06822890043258667,
    -0.06037856638431549,
    -0.04865343123674393,
    -0.032734472304582596,
    -0.012384352274239063,
    0.012538252398371696,
    0.042066626250743866,
    0.07611683756113052,
    0.11448182910680771,
    0.15682920813560486,
    0.20270287990570068,
    0.25152862071990967,
    0.30262377858161926,
    0.355210542678833,
    0.40843257308006287,
    0.46137505769729614,
    0.51308673620224,
    0.5626039505004883,
    0.6089754700660706,
    0.6512883305549622,
    0.6886915564537048,
    0.720420777797699,
    0.7458184957504272,
    0.7643529772758484,
    0.7756332755088806,
    0.7794201970100403,
    0.7756332755088806,
    0.7643529772758484,
    0.7458184957504272,
    0.720420777797699,
    0.6886915564537048,
    0.6512883305549622,
    0.6089754700660706,
    0.5626039505004883,
    0.51308673620224,
    0.46137505769729614,
    0.40843257308006287,
    0.355210542678833,
    0.30262377858161926,
    0.25152862071990967,
    0.20270287990570068,
    0.15682920813560486,
    0.11448182910680771,
    0.07611683756113052,
    0.042066626250743866,
    0.012538252398371696,
    -0.012384352274239063,
    -0.032734472304582596,
    -0.04865343123674393,
    -0.06037856638431549,
    -0.06822890043258667,
    -0.07258912175893784,
    -0.0738925039768219,
    -0.07260339707136154,
    -0.0692000463604927,
    -0.064158134162426,
    -0.05793576315045357,
    -0.050960298627614975,
    -0.043617404997348785,
    -0.03624257445335388,
    -0.02911534160375595,
    -0.022456131875514984,
    -0.016425788402557373,
    -0.011127560399472713,
    -0.006611250340938568,
    -0.0028792729135602713,
    0.00010584338451735675,
    0.002412923611700535,
    0.004133045207709074,
    0.005370331462472677,
    0.006233078893274069,
    0.006825620774179697,
    0.007241279352456331,
    0.00755667919293046,
    0.007827633991837502
];


#[cfg(test)]
mod tests {
    #[test]
    fn floaty() {
        use super::to_float;
        assert_eq!(0., to_float(-128));
        assert_eq!(64. / 255., to_float(-64));
        assert_eq!(128. / 255., to_float(0));
        assert_eq!(1., to_float(127));
    }
}
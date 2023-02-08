mod constants;
mod fourier;
mod io_utils;

use hound::{SampleFormat, WavSpec};
use std::i16;

use crate::io_utils::*;

fn main() {
    // spec
    let spec = WavSpec {
        channels: constants::DEFAULT_CHANNELS,
        sample_rate: constants::DEFAULT_SAMPLE_RATE,
        bits_per_sample: constants::DEFAULT_BITS_PER_SAMPLE,
        sample_format: constants::DEFAULT_SAMPLE_FORMAT,
    };

    // sample generator (can be a simple call to an array)
    let sg = simple_sg;
    let sg = simple_toy_sample;

    // duration of the sample is given in terms of the sample rate
    let seconds = 5;
    let duration: usize = (spec.sample_rate * seconds) as usize;

    // write to filename with spec and sample generator
    let _ = write("sine.wav", spec, sg, duration);

    // give a filename and everything is get from the reader struct
    let filename = "sample2.wav";
    let mut reader = read(filename);

    let spec = reader.spec();
    let duration = reader.duration();
    let length = reader.len();
    let duration_in_secs = duration / spec.sample_rate;
    let sqr_sum = reader.samples::<i16>().fold(0.0, |sqr_sum, s| {
        let sample = s.unwrap() as f64;
        sqr_sum + sample * sample
    });

    println!(
        "channels: {}, sample_rate: {}, bits_per_sample: {}, sample_format: {:?}",
        spec.channels, spec.sample_rate, spec.bits_per_sample, spec.sample_format
    );
    println!(
        "duration: {}\nlenght: {}\nduration in seconds: {}",
        duration, length, duration_in_secs
    );
    println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
}

pub fn simple_toy_sample(t: usize) -> i16 {
    let value = t % (constants::DEFAULT_SAMPLE_RATE as usize);

    let coeffs: Vec<f32> = (0..(constants::DEFAULT_SAMPLE_RATE / 10))
        .map(|x| (x % 43) as f32)
        .collect::<Vec<f32>>();
    let value = fourier::evaluate(value as f32, &coeffs, None);
    let amplitude = (i16::MAX / ((t + 1) as i16)) as f32;

    (value * amplitude) as i16
}

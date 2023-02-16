use trombone::{constants, io_utils, sample_generator};

use hound::WavSpec;
use std::i16;

fn main() {
    example_write();

    example_read();
}

pub fn example_read() {
    // give a filename and everything is get from the reader struct
    let filename = "sample2.wav";
    let mut reader = io_utils::read(filename);

    let spec = reader.spec();
    let duration = reader.duration();
    let length = reader.len();
    let duration_in_secs = io_utils::duration_in_secs(&reader);

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

pub fn example_write() {
    // define a spec
    let spec = WavSpec {
        channels: constants::DEFAULT_CHANNELS,
        sample_rate: constants::DEFAULT_SAMPLE_RATE,
        bits_per_sample: constants::DEFAULT_BITS_PER_SAMPLE,
        sample_format: constants::DEFAULT_SAMPLE_FORMAT,
    };

    // sample generator (must implement the SampleGenerator trait)
    let mut sg = sample_generator::DummySGi16::new();

    // duration of the sample is given in terms of the sample rate
    let seconds = 5;
    let duration: usize = (spec.sample_rate * seconds) as usize;

    // write to filename with spec and sample generator
    let _ = io_utils::write("sine.wav", spec, &mut sg, duration);
}

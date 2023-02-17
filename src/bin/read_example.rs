use trombone::io_utils;

use std::i16;

fn main() {
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

use crate::constants::*;
use hound::{Sample, SampleFormat, WavIntoSamples, WavReader};
use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io::BufReader;

struct SGi16 {
    samples: WavIntoSamples<BufReader<File>, i16>,
}

impl SGi16 {
    pub fn new(reader: WavReader<BufReader<File>>) -> Self {
        let samples: WavIntoSamples<BufReader<File>, i16> = reader.into_samples();

        SGi16 { samples }
    }
}

impl SampleGenerator<i16> for SGi16 {
    fn sample(n: usize) -> i16 {
        0
    }
}

pub fn simple_sg(t: usize) -> i16 {
    let t32 = t as f32;
    let sample: f32;
    let amplitude: f32;

    if t > 44100 {
        sample = ((t32 / 44100.0) * 440.0 * 1.0 * PI).sin();
        amplitude = i16::MAX as f32;
    } else {
        sample = ((t32 / 44100.0) * 440.0 * 2.0 * PI).sin();
        amplitude = (i16::MAX / 2) as f32;
    }

    return (sample * amplitude) as i16;
}

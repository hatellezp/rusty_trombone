use hound::{WavIntoSamples, WavReader, WavSpec, WavWriter};

use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io::BufReader;

pub type SampleGeneratorI16 = fn(usize) -> i16;

pub fn read(filename: &str) -> WavReader<BufReader<File>> {
    WavReader::open(filename).unwrap()
}

pub fn write(
    filename: &str,
    spec: WavSpec,
    sg: SampleGeneratorI16,
    duration: usize,
) -> Result<(), hound::Error> {
    let mut writer = WavWriter::create(filename, spec).unwrap();

    for t in 0..duration {
        writer.write_sample(sg(t)).unwrap();
    }

    writer.finalize()
}

pub fn duration_in_secs(reader: &WavReader<BufReader<File>>) -> usize {
    (reader.duration() / reader.spec().sample_rate) as usize
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

// TODO
/*
pub fn create_sg(reader: WavReader<BufReader<File>>) -> SampleGeneratorI16 {
    let into_samples: WavIntoSamples<BufReader<File>, i16> = reader.into_samples();

    let sg = |t: usize| match into_samples.next() {
        Some(res) => res.unwrap(),
        None => 0 as i16,
    };
}
*/

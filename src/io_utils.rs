use hound::{WavReader, WavSamples, WavSpec, WavWriter};

use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io::BufReader;

pub fn read(filename: &str) -> WavReader<BufReader<File>> {
    WavReader::open(filename).unwrap()
}

pub fn write<SG: Fn(usize) -> i16>(
    filename: &str,
    spec: WavSpec,
    sg: SG,
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

pub fn create_sample_generator_from_reader(
    reader: &mut WavReader<BufReader<File>>,
) -> impl FnMut(usize) -> i16 + '_ {
    let mut samples: WavSamples<BufReader<File>, i16> = reader.samples();

    let samples_length = samples.len();

    let f = move |t: usize| {
        if t < samples_length {
            samples.next().unwrap().unwrap()
        } else {
            0 as i16
        }
    };

    f
}

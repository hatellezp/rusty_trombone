use crate::sample_generator::SampleGenerator;

use hound::{Sample, WavReader, WavSpec, WavWriter};

use std::fs::File;
use std::i16;
use std::io::BufReader;

pub fn read(filename: &str) -> WavReader<BufReader<File>> {
    WavReader::open(filename).unwrap()
}

pub fn write<SG: SampleGenerator<i16>>(
    filename: &str,
    spec: WavSpec,
    sg: &mut SG,
    duration: usize,
) -> Result<(), hound::Error> {
    let mut writer = WavWriter::create(filename, spec).unwrap();

    for _ in 0..duration {
        let value = match sg.next() {
            Some(v) => v,
            _ => 0,
        };

        writer.write_sample(value).unwrap();
    }

    writer.finalize()
}

pub fn duration_in_secs(reader: &WavReader<BufReader<File>>) -> usize {
    (reader.duration() / reader.spec().sample_rate) as usize
}

pub fn to_vec<S: Sample>(
    mut reader: WavReader<BufReader<File>>,
    duration: Option<usize>,
) -> Vec<S> {
    let mut v = reader
        .samples::<S>()
        .map(|x| x.unwrap() as S)
        .collect::<Vec<S>>();
    v.truncate(duration.unwrap_or(v.len()));
    v
}

pub fn read_batch<S: Sample>(
    filenames: &[&str],
    duration: Option<usize>,
) -> Vec<(WavSpec, Vec<S>)> {
    filenames
        .iter()
        .map(|filename| read(filename))
        .map(|reader| (reader.spec(), to_vec(reader, duration)))
        .collect::<Vec<(WavSpec, Vec<S>)>>()
}

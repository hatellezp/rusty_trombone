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
    allow_dirty: bool,
) -> Vec<(String, WavSpec, Vec<S>)> {
    let wav_reader_filter =
        |reader: &WavReader<BufReader<File>>| match (allow_dirty, duration.is_some()) {
            (true, true) => {
                if (duration.unwrap() as u32) > reader.duration() {
                    None
                } else {
                    Some(())
                }
            }
            _ => Some(()),
        };

    let mut values = vec![];

    for index in 0..filenames.len() {
        let filename = filenames[index];
        let reader = read(filename);

        let is_good = wav_reader_filter(&reader).is_some();

        if is_good {
            let spec = reader.spec().clone();
            let data = to_vec::<S>(reader, duration);

            values.push((filename.to_string(), spec, data))
        }
    }

    values
}

use crate::sample_generator::SampleGenerator;

use hound::{WavReader, WavSpec, WavWriter};

use std::fs::File;
use std::i16;
use std::io::BufReader;

// TODO: change BufReader<File> to something that accept generics
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

use hound::{WavReader, WavSpec, WavWriter};

use std::fs::File;
use std::i16;
use std::io::BufReader;

// TODO: change BufReader<File> to something that accept generics

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

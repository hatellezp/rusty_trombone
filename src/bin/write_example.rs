use trombone::{constants, io_utils, sample_generator};

use hound::WavSpec;

fn main() {
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

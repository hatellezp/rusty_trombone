use hound::SampleFormat;

// constants from the hound module
pub const DEFAULT_CHANNELS: u16 = 2;
pub const DEFAULT_SAMPLE_RATE: u32 = 44100;
pub const DEFAULT_BITS_PER_SAMPLE: u16 = 16;
pub const DEFAULT_SAMPLE_FORMAT: SampleFormat = SampleFormat::Int;

// all samples are going to be of length 3min
// meaning 3 * 60 * sample_rate
pub const DEFAULT_DURATION: u32 = 3 * 60 * DEFAULT_SAMPLE_RATE;

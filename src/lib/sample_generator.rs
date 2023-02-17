use hound::{Sample, WavIntoSamples, WavReader};
use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io::BufReader;

pub trait SampleGenerator<S: Sample> {
    fn next(&mut self) -> Option<S>;
}

pub struct SGFromSamples<S: Sample> {
    samples: WavIntoSamples<BufReader<File>, S>,
}

impl<S: Sample> SGFromSamples<S> {
    pub fn new(reader: WavReader<BufReader<File>>) -> Self {
        let samples: WavIntoSamples<BufReader<File>, S> = reader.into_samples();

        SGFromSamples { samples }
    }
}

impl<S: Sample> SampleGenerator<S> for SGFromSamples<S> {
    fn next(&mut self) -> Option<S> {
        let res = self.samples.next();

        match res {
            Some(value) => match value {
                Ok(value) => Some(value),
                _ => None,
            },
            _ => None,
        }
    }
}

pub struct DummySGi16 {
    t: usize,
}

impl DummySGi16 {
    pub fn new() -> DummySGi16 {
        DummySGi16 { t: 0 }
    }

    pub fn get(&self, t: usize) -> i16 {
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

        (sample * amplitude) as i16
    }

    fn increase(&mut self) {
        self.t += 1;
    }
}

impl SampleGenerator<i16> for DummySGi16 {
    fn next(&mut self) -> Option<i16> {
        let t = self.t;
        self.increase();

        Some(self.get(t))
    }
}

pub struct SGfromArray<S: Sample> {
    index: usize,
    data: Vec<S>,
}

impl<S: Sample + Copy> SGfromArray<S> {
    pub fn new(data: &[S]) -> SGfromArray<S> {
        SGfromArray {
            index: 0,
            data: data.iter().map(|x| *x).collect::<Vec<S>>(),
        }
    }
}

impl SampleGenerator<i16> for SGfromArray<i16> {
    fn next(&mut self) -> Option<i16> {
        if self.index >= self.data.len() {
            None
        } else {
            self.index += 1;
            Some(self.data[self.index - 1])
        }
    }
}

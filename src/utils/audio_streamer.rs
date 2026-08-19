use std::num::NonZero;
use std::sync::mpsc;

use rodio::Source;

pub struct AudioStreamer {
    channels: u16,
    sample_rate: u32,
    rx: mpsc::Receiver<f32>,
}
impl Iterator for AudioStreamer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.rx.recv().ok()
    }
}

impl Source for AudioStreamer {
    fn channels(&self) -> rodio::ChannelCount {
        NonZero::new(self.channels).expect("Audio sources need at least one channel")
    }
    fn sample_rate(&self) -> rodio::SampleRate {
        NonZero::new(self.sample_rate).expect("Audio sources requires sample rate")
    }
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl AudioStreamer {
    pub fn new(rx: mpsc::Receiver<f32>, sample_rate: u32, channels: u16) -> Self {
        Self { channels: channels, sample_rate: sample_rate, rx: rx,}
    }
}
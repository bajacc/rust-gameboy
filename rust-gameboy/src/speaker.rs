
use std::sync::{Arc, atomic::Ordering};
use atomic_float::AtomicF32;

use rodio::{OutputStream, Sink, Source};

use crate::gb::GameBoy;

#[derive(Default, Clone)]
pub struct SoundSource {
    pub output1: Arc<AtomicF32>,
    pub output2: Arc<AtomicF32>,
    output_id: usize,
}

impl SoundSource {
    pub fn new() -> Self {
        SoundSource::default()
    }

    pub fn update(&mut self, output1: f32, output2: f32) {
        self.output1.store(output1, Ordering::Relaxed);
        self.output2.store(output2, Ordering::Relaxed);
    }
}

impl Iterator for SoundSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.output_id {
            0 => self.output1.load(Ordering::Relaxed),
            1 => self.output2.load(Ordering::Relaxed),
            _ => panic!("unreachable output id")
        };

        self.output_id = (self.output_id + 1) % 2;
        Some(result)
    }
}


impl Source for SoundSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        2
    }

    fn sample_rate(&self) -> u32 {
        441000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

pub struct Speaker {
    stream: OutputStream,
    sink: Sink,
    sound_source: SoundSource
}


impl Speaker {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let speaker = Speaker {
            stream: _stream,
            sink: Sink::try_new(&stream_handle).unwrap(),
            sound_source: SoundSource::new(),
        };
        stream_handle.play_raw(speaker.sound_source.clone().convert_samples()).unwrap();
        speaker
    }

    pub fn update(&mut self, gb: &GameBoy) {
        self.sound_source.update(gb.mmu.sound.so1_output, gb.mmu.sound.so2_output);
    }
}
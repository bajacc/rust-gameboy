use std::sync::mpsc::SyncSender;

use rodio::{OutputStream, OutputStreamHandle, Source};
use std::sync::mpsc::{sync_channel, Receiver};

use crate::gb::GameBoy;

pub struct SoundSource {
    pub sample_rate: u32,
    receiver: Receiver<f32>,
}

impl Iterator for SoundSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.receiver.recv().unwrap())
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
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

const CYCLE_PER_SECOND: u32 = 1 << 20;
const CHANNEL_SIZE: usize = 1 << 12;

pub struct Speaker {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    cycle_per_sample: u32,
    counter: u32,
    sender: SyncSender<f32>,
}

impl Speaker {
    pub fn new(sample_rate: u32) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let (sender, recv) = sync_channel(CHANNEL_SIZE);

        //send to channel to avoid blocking the main thread (why?)
        for _ in 0..4 {
            sender.send(0.0).unwrap();
        }
        let sound_source = SoundSource {
            sample_rate: sample_rate,
            receiver: recv,
        };
        stream_handle.play_raw(sound_source).unwrap();
        Speaker {
            stream: _stream,
            stream_handle: stream_handle,
            cycle_per_sample: CYCLE_PER_SECOND / sample_rate,
            sender: sender,
            counter: 0,
        }
    }

    pub fn cycle(&mut self, gb: &GameBoy) {
        if self.counter == self.cycle_per_sample {
            self.counter = 0;

            let (left, right) = gb.mmu.sound.get_sample();
            self.sender.send(left).unwrap();
            self.sender.send(right).unwrap();
        }
        self.counter += 1;
    }
}

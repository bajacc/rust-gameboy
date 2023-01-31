use rodio::{OutputStream, Sink, buffer::SamplesBuffer, Source};

use crate::{sound::{Sound, self}, gb::GameBoy};

impl Iterator for Sound {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.so1_output)
    }
}


impl Source for Sound {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        1 << 20
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

pub struct Speaker {
    stream: OutputStream,
    sink: Sink
}


impl Speaker {
    pub fn new(sound: &mut Sound) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        stream_handle.play_raw(sound.convert_samples());
        Speaker {
            stream: _stream,
            sink: Sink::try_new(&stream_handle).unwrap()
        }
    }

    // pub fn play(&mut self, gb: &GameBoy) {
    //     let sound = &gb.mmu.sound;
    //     let source = SamplesBuffer::new(1, 1 << 20, vec![1u16, sound.so1_output]);
    //     self.sink.append(source);
    // }
}
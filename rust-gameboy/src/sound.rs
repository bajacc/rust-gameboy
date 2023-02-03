use crate::mmu;

macro_rules! bit {
    ($x:expr, $pos:expr) => {
        ($x & (1 << ($pos as u8))) != 0
    };
}

fn to_f32(v: u8) -> f32 {
    return (v as f32 / 7.5) - 1.0;
}

// Step   Length Ctr  Vol Env     Sweep
// ---------------------------------------
// 0      Clock       -           -
// 1      -           -           -
// 2      Clock       -           Clock
// 3      -           -           -
// 4      Clock       -           -
// 5      -           -           -
// 6      Clock       -           Clock
// 7      -           Clock       -
// ---------------------------------------
// Rate   256 Hz      64 Hz       128 Hz
#[derive(Default)]
struct FrameSequencer {
    pub length_ctr: bool,
    pub vol_env: bool,
    pub sweep: bool,
    counter: usize,
}

const CYCLE_PER_SECOND: usize = 1 << 20;
const CYCLE_PER_512HZ: usize = CYCLE_PER_SECOND / 512;

impl FrameSequencer {
    pub fn cycle(&mut self) {
        self.length_ctr = false;
        self.vol_env = false;
        self.sweep = false;

        self.counter += 1;

        if self.counter % CYCLE_PER_512HZ != 0 {
            return;
        }

        let step = self.counter / CYCLE_PER_512HZ;
        if step == 8 {
            self.counter = 0;
        }

        if step % 2 == 0 {
            self.length_ctr = true;
        }
        if step == 7 {
            self.vol_env = true;
        }
        if step == 2 || step == 6 {
            self.sweep = true;
        }
    }
}

#[derive(Default)]
struct Envelope {
    pub volume: u8,
    pub period: u8,
    pub increment: bool,
    pub nrx2: u8,
    period_counter: u8,
}

impl Envelope {
    pub fn cycle(&mut self, frame_sequencer: &FrameSequencer) {
        if !frame_sequencer.sweep || self.period == 0 {
            return;
        }

        if self.period_counter != 0 {
            self.period_counter -= 1;
        }

        if self.period_counter != 0 {
            return;
        }

        self.period_counter = self.period;

        match self.increment {
            false if self.volume > 0 => self.volume -= 1,
            true if self.volume < 0xf => self.volume += 1,
            _ => ()
        }
    }

    pub fn set_nrx2(&mut self, value: u8) {
        self.volume = value >> 4;
        self.period = value & 0x7;
        self.increment = bit!(value, 3);
        self.nrx2 = value;
    }
}

#[derive(Default)]
struct Sweep {
    pub nr10: u8,
    pub shift: u8,
    pub period: u8,
    pub increment: bool,

    pub enable: bool,
    pub freq: u16,
    period_counter: u8,
}

impl Sweep {
    pub fn cycle(&mut self, frame_sequencer: &FrameSequencer) {
        if !frame_sequencer.vol_env && self.enable {
            return;
        }

        if self.period_counter != 0 {
            self.period_counter -= 1;
            return;
        }

        if self.period_counter != 0 {
            return;
        }
        self.period_counter = match self.period {
            0 => 8,
            _ => self.period,
        };

        let new_freq = self.compute_next_freq();
        if new_freq > 0 && new_freq < 2048 {
            self.freq = new_freq as u16;
            self.compute_next_freq();
        }
        
    }

    fn compute_next_freq(&mut self) -> i16 {
        let shifted = (self.freq >> self.shift) as i16;
        let new_freq = self.freq as i16 + match self.increment {
            false => -shifted,
            true => shifted,
        };
        if new_freq >= 2048 {
            self.enable = false;
        }
        new_freq
    }

    pub fn set_nr10(&mut self, value: u8) {
        self.nr10 = value;
        self.period = (value >> 4) & 0x7;
        self.shift = value & 0x7;
        self.increment = !bit!(value, 3);
    }

    pub fn trigger(&mut self, freq: u16) {
        self.freq = freq;
        self.period_counter = match self.period {
            0 => 8,
            _ => self.period,
        };
        self.enable = self.period != 0 || self.shift != 0;
        if self.shift != 0 {
            self.compute_next_freq();
        }
    }
}

#[derive(Default)]
struct Wave {
    enable: bool,
    dac_power: bool,
    length: u8,
    volume: u8,
    nr33: u8,
    nr34: u8,
    ram: [u8; 0x10],

    counter: usize,
    len_counter: usize,

    position: usize,
}

impl Wave {
    const VOLUME_SHIFT: [u8; 4] = [4, 0, 1, 2];

    pub fn cycle(&mut self, frame_sequencer: &FrameSequencer) {

        if !self.enable {
            return;
        }        

        if self.counter != 0 {
            self.counter -= 1;
            if self.counter == 0 {
                self.counter = self.period();
                self.position = (self.position + 1) % (self.ram.len() * 2);
            }
        }

        if frame_sequencer.length_ctr && self.len_counter != 0 {
            self.len_counter -= 1;
            if self.len_counter == 0 && self.nr34 & 0x40 != 0 {
                self.enable = false;
            }
        }
    }

    fn get_sample(&self) -> u8 {
        if !self.enable {
            return 0;
        }
        let mut output = self.ram[self.position / 2];
        output >>= (self.position & 1) * 4;
        output &= 0x0f;
        output >>= Wave::VOLUME_SHIFT[self.volume as usize];
        return output;
    }

    fn trigger(&mut self) {
        if self.nr34 & 0x80 == 0 {
            return;
        }

        self.enable = true;
        if self.len_counter == 0 {
            self.len_counter = 256 - self.length as usize;
        }
        self.counter = self.period();
        self.position = 0;
    }

    fn period(&mut self) -> usize {
        let x = self.nr33 as usize | (self.nr34 as usize & 7) << 8;
        return 2 * (2048 - x);
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0 => (self.dac_power as u8) << 7,
            1 => self.length,
            2 => self.volume << 5,
            3 => self.nr33,
            4 => self.nr34,
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0 => {
                self.dac_power = value & 0x80 != 0;
                if !self.dac_power {
                    self.enable = false;
                }
            }
            1 => self.length = value,
            2 => self.volume = (value >> 5) & 3,
            3 => self.nr33 = value,
            4 => {
                self.nr34 = value;
                self.trigger();
            }
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }
}


#[derive(Default)]
struct Square {
    envelope: Envelope,
    sweep: Option<Sweep>,
    enable: bool,
    nr21: u8,
    nr23: u8,
    nr24: u8,

    counter: usize,
    len_counter: usize,

    position: usize,
}

pub const WAVEFORM: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

impl Square {

    pub fn cycle(&mut self, frame_sequencer: &FrameSequencer) {
        self.envelope.cycle(frame_sequencer);
        if let Some(sweep) = self.sweep.as_mut() {
            sweep.cycle(frame_sequencer);
            self.enable |= sweep.enable;
        }

        if !self.enable {
            return;
        }

        if self.counter != 0 {
            self.counter -= 1;
            if self.counter == 0 {
                self.counter = self.period();
                self.position = (self.position + 1) % 8;
            }
        }

        if frame_sequencer.length_ctr && self.len_counter != 0 {
            self.len_counter -= 1;
            if self.len_counter == 0 && self.nr24 & 0x40 != 0 {
                self.enable = false;
            }
        }
    }

    pub fn get_sample(&self) -> u8 {
        if !self.enable {
            return 0;
        }
        let duty = self.nr21 >> 6;
        return WAVEFORM[duty as usize][self.position] * self.envelope.volume;
    }

    fn trigger(&mut self) {
        if self.nr24 & 0x80 == 0 {
            return;
        }

        self.enable = true;
        if self.len_counter == 0 {
            let length = self.nr21 & 0x3f;
            self.len_counter = 64 - length as usize;
        }

        let freq = self.freq();
        if let Some(sweep) = self.sweep.as_mut() {
            sweep.trigger(freq);
        }
        self.counter = self.period();
    }

    fn freq(&self) -> u16 {
        self.nr23 as u16 | (self.nr24 as u16 & 7) << 8
    }

    fn period(&self) -> usize {
        let x = match self.sweep.as_ref() {
            Some(sweep) => sweep.freq,
            _ => self.freq(),
        };
        return 2 * (2048 - x as usize);
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0 => match self.sweep.as_ref() {
                Some(sweep) => sweep.nr10,
                _ => mmu::NO_DATA
            },
            1 => self.nr21,
            2 => self.envelope.nrx2,
            3 => self.nr23,
            4 => self.nr24,
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0 => if let Some(sweep) = self.sweep.as_mut() {
                sweep.set_nr10(value);
            }
            1 => self.nr21 = value,
            2 => {
                self.envelope.set_nrx2(value);
                if value & 0xf8 == 0 {
                    self.enable = false;
                }
            },
            3 => self.nr23 = value,
            4 => {
                self.nr24 = value;
                self.trigger();
            }
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }
}

#[derive(Default)]
struct Noise {
    envelope: Envelope,
    enable: bool,
    nr43: u8,
    nr44: u8,
    lfsr: u16,

    len_counter: usize,
    length: u8,

    counter: usize,
}

impl Noise {

    pub fn cycle(&mut self, frame_sequencer: &FrameSequencer) {
        self.envelope.cycle(frame_sequencer);

        if !self.enable {
            return;
        }

        if self.counter != 0 {
            self.counter -= 1;
            if self.counter == 0 {
                self.counter = self.period();

                let xor = (self.lfsr & 1) ^ ((self.lfsr & 2) >> 1);
                self.lfsr = (self.lfsr >> 1) | (xor << 14);

                if bit!(self.nr43, 3) {
                    self.lfsr &= !(1 << 6);
                    self.lfsr |= xor << 6;
                }
            }
        }

        if frame_sequencer.length_ctr && self.len_counter != 0 {
            self.len_counter -= 1;
            if self.len_counter == 0 && self.nr44 & 0x40 != 0 {
                self.enable = false;
            }
        }
    }

    fn get_sample(&self) -> u8 {
        if !self.enable {
            return 0;
        }
        return match bit!(self.lfsr, 1) {
            true => 0,
            false => self.envelope.volume,
        };
    }

    fn trigger(&mut self) {
        if self.nr44 & 0x80 == 0 {
            return;
        }

        self.enable = true;
        self.lfsr = 0x7fff;
        if self.len_counter == 0 {
            self.len_counter = 64 - self.length as usize;
        }
        self.counter = self.period();
    }

    fn period(&self) -> usize {
        let divisor_code = (self.nr43 & 7) as usize;
        let shift = ((self.nr43 >> 4) & 7) as usize;
        let divisor = match divisor_code {
            0 => 1,
            _ => divisor_code << 1,
        };
        divisor << (shift + 1) // todo factor 2?
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0 => mmu::NO_DATA,
            1 => self.length,
            2 => self.envelope.nrx2,
            3 => self.nr43,
            4 => self.nr44,
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0 => (),
            1 => self.length = value & 0x3f,
            2 => {
                self.envelope.set_nrx2(value);
                if value & 0xf8 == 0 {
                    self.enable = false;
                }
            },
            3 => self.nr43 = value,
            4 => {
                self.nr44 = value;
                self.trigger();
            }
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }
}

enum Nr51 {
    Sound4ToSo2 = 7,
    Sound3ToSo2 = 6,
    Sound2ToSo2 = 5,
    Sound1ToSo2 = 4,
    Sound4ToSo1 = 3,
    Sound3ToSo1 = 2,
    Sound2ToSo1 = 1,
    Sound1ToSo1 = 0,
}

#[derive(Default)]
pub struct Sound {
    frame_sequencer: FrameSequencer,
    square1: Square,
    square2: Square,
    wave: Wave,
    noise: Noise,

    nr50: u8,
    nr51: u8,
    enable: bool,
    so1_volume: u16,
    so2_volume: u16,
    pub position: usize,
}

impl Sound {
    pub fn new() -> Self {
        let mut sound = Sound::default();
        sound.square1.sweep = Some(Sweep::default());
        sound
    }

    pub fn cycle(&mut self) {
        self.frame_sequencer.cycle();
        self.wave.cycle(&self.frame_sequencer);
        self.square1.cycle(&self.frame_sequencer);
        self.square2.cycle(&self.frame_sequencer);
        self.noise.cycle(&self.frame_sequencer);        
    }

    pub fn get_sample(&self) -> (f32, f32) {
        let mut so1_output = 0.0;
        let mut so2_output = 0.0;
        
        let square1_out = to_f32(self.square1.get_sample());
        let square2_out = to_f32(self.square2.get_sample());
        let wave_out = to_f32(self.wave.get_sample());
        let noise_out = to_f32(self.noise.get_sample());
        if bit!(self.nr51, Nr51::Sound1ToSo1) {
            so1_output += square1_out;
        }
        if bit!(self.nr51, Nr51::Sound1ToSo2) {
            so2_output += square1_out;
        }
        if bit!(self.nr51, Nr51::Sound2ToSo1) {
            so1_output += square2_out;
        }
        if bit!(self.nr51, Nr51::Sound2ToSo2) {
            so2_output += square2_out;
        }
        if bit!(self.nr51, Nr51::Sound3ToSo1) {
            so1_output += wave_out;
        }
        if bit!(self.nr51, Nr51::Sound3ToSo2) {
            so2_output += wave_out;
        }
        if bit!(self.nr51, Nr51::Sound4ToSo1) {
            so1_output += noise_out;
        }
        if bit!(self.nr51, Nr51::Sound4ToSo2) {
            so2_output += noise_out;
        }

        so1_output *= self.so1_volume as f32 + 1.0;
        so2_output *= self.so2_volume as f32 + 1.0;

        so1_output /= 32.0;
        so2_output /= 32.0;

        return (so1_output, so2_output);
    }

    fn set_nr50(&mut self, v: u8) {
        self.nr50 = v;
        self.so1_volume = (v & 0x07) as u16;
        self.so2_volume = ((v >> 4) & 0x07) as u16;
    }

    fn get_nr52(&self) -> u8 {
        return (self.enable as u8) << 7 
        | (self.wave.enable as u8) << 3 
        | (self.wave.enable as u8) << 2 
        | (self.square2.enable as u8) << 1 
        | (self.square1.enable as u8);
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff10..=0xff14 => self.square1.read(addr - 0xff10),
            0xff15..=0xff19 => self.square2.read(addr - 0xff15),
            0xff1a..=0xff1e => self.wave.read(addr - 0xff1a),
            0xff1f..=0xff23 => self.noise.read(addr - 0xff1f),
            0xff24 => self.nr50,
            0xff25 => self.nr51,
            0xff26 => self.get_nr52(),
            0xff27..=0xff2f => mmu::NO_DATA,
            0xff30..=0xff3f => self.wave.ram[addr as usize - 0xff30],
            _ => mmu::NO_DATA,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff10..=0xff14 => self.square1.write(addr - 0xff10, value),
            0xff15..=0xff19 => self.square2.write(addr - 0xff15, value),
            0xff1a..=0xff1e => self.wave.write(addr - 0xff1a, value),
            0xff1f..=0xff23 => self.noise.write(addr - 0xff1f, value),
            0xff24 => self.set_nr50(value),
            0xff25 => self.nr51 = value,
            0xff26 => self.enable = value & 0x80 != 0,
            0xff27..=0xff2f => (),
            0xff30..=0xff3f => self.wave.ram[addr as usize - 0xff30] = value,
            _ => (),
        }
    }
}
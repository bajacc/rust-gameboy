use crate::mmu;

macro_rules! bit {
    ($x:expr, $pos:expr) => {
        ($x & (1 << ($pos as u8))) != 0
    };
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
    }
}

#[derive(Default)]
struct Sweep {
    pub shift: u8,
    pub period: u8,
    pub increment: bool,

    pub enable: bool,
    pub freq: u16,
    period_counter: u8,
}

impl Sweep {
    pub fn cycle(&mut self, frame_sequencer: &FrameSequencer) {
        if !frame_sequencer.vol_env {
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
    pub output: f32
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
                self.reset_counter();
                self.position = (self.position + 1) % (self.ram.len() * 2);
                
                let mut output = self.ram[self.position / 2];
                output >>= (self.position & 1) * 4;
                output &= 0x0f;
                output >>= Wave::VOLUME_SHIFT[self.volume as usize];

                self.output = (output as f32 / 7.5) - 1.0;
            }
        }

        if frame_sequencer.length_ctr && self.len_counter != 0 {
            self.len_counter -= 1;
            if self.len_counter == 0 && self.nr34 & 0x40 != 0 {
                self.enable = false;
                self.output = 0.0;
            }
        }
    }

    fn check_trigger(&mut self) {
        if self.nr34 & 0x80 == 0 {
            return;
        }

        self.enable = true;
        if self.len_counter == 0 {
            self.len_counter = 256 - self.length as usize;
        }
        self.reset_counter();
        self.position = 0;
    }

    fn reset_counter(&mut self) {
        let x = self.nr33 as usize | (self.nr34 as usize & 7) << 8;
        // freq = 65536/(2048 - x) in Hz
        // period = (4194304 / freq) in s

        // period in cycle
        self.counter = 2 * (2048 - x); // todo: mutiply by 2 or 64?
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff1a => (self.dac_power as u8) << 7,
            0xff1b => self.length,
            0xff1c => self.volume << 5,
            0xff1d => self.nr33,
            0xff1e => self.nr34,
            0xff30..=0xff3f => self.ram[addr as usize - 0xff30],
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff1a => {
                self.dac_power = value & 0x80 != 0;
                if !self.dac_power {
                    self.enable = false;
                }
            }
            0xff1b => self.length = value,
            0xff1c => self.volume = (value >> 5) & 3,
            0xff1d => self.nr33 = value,
            0xff1e => {
                self.nr34 = value;
                self.check_trigger();
            }
            0xff30..=0xff3f => self.ram[addr as usize - 0xff30] = value,
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }
}


#[derive(Default)]
struct Square2 {
    envelope: Envelope,
    enable: bool,
    dac_power: bool,
    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,

    counter: usize,
    len_counter: usize,

    position: usize,
    pub output: f32
}

pub const WAVEFORM: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

impl Square2 {
    pub fn cycle(&mut self, frame_sequencer: &FrameSequencer) {
        self.envelope.cycle(frame_sequencer);

        if !self.enable {
            return;
        }

        if self.counter != 0 {
            self.counter -= 1;
            if self.counter == 0 {
                self.counter = self.period();
                
                let duty = self.nr21 >> 6;

                let mut output = WAVEFORM[duty as usize][self.position];
                output *= self.envelope.volume;
                
                self.position = (self.position + 1) % 8;
                self.output = (output as f32 / 7.5) - 1.0;
            }
        }

        if frame_sequencer.length_ctr && self.len_counter != 0 {
            self.len_counter -= 1;
            if self.len_counter == 0 && self.nr24 & 0x40 != 0 {
                self.enable = false;
                self.output = 0.0;
            }
        }
    }

    fn check_trigger(&mut self) {
        if self.nr24 & 0x80 == 0 {
            return;
        }

        self.enable = true;
        if self.len_counter == 0 {
            let length = self.nr21 & 0x3f;
            self.len_counter = 64 - length as usize;
        }
        self.counter = self.period();
    }

    fn period(&self) -> usize {
        let x = self.nr23 as usize | (self.nr24 as usize & 7) << 8;
        // freq = 65536/(2048 - x) in Hz
        // period = (4194304 / freq) in s

        // period in cycle
        return 2 *  (2048 - x); // todo: mutiply by 2 or 64?
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff15 => mmu::NO_DATA,
            0xff16 => self.nr21,
            0xff17 => self.nr22,
            0xff18 => self.nr23,
            0xff19 => self.nr24,
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff15 => (),
            0xff16 => self.nr21 = value,
            0xff17 => {
                self.nr22 = value;
                if self.nr22 & 0xf8 == 0 {
                    self.enable = false;
                }
                self.envelope.set_nrx2(value);
            },
            0xff18 => self.nr23 = value,
            0xff19 => {
                self.nr24 = value;
                self.check_trigger();
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
    wave: Wave,
    square2: Square2,

    nr50: u8,
    nr51: u8,
    enable: bool,
    so1_volume: u16,
    so2_volume: u16,

    pub so1_output: f32,
    pub so2_output: f32,
    pub position: usize,
}

impl Sound {
    pub fn new() -> Self {
        Sound::default()
    }

    pub fn cycle(&mut self) {
        self.frame_sequencer.cycle();
        self.wave.cycle(&self.frame_sequencer);
        self.square2.cycle(&self.frame_sequencer);

        self.so1_output = 0.0;
        self.so2_output = 0.0;
        // if bit!(self.nr51, Nr51::Sound3ToSo1) {
        //     self.so1_output += self.wave.output;
        // }
        // if bit!(self.nr51, Nr51::Sound3ToSo2) {
        //     self.so2_output += self.wave.output;
        // }
        if bit!(self.nr51, Nr51::Sound2ToSo1) {
            self.so1_output += self.square2.output;
        }
        if bit!(self.nr51, Nr51::Sound2ToSo2) {
            self.so2_output += self.square2.output;
        }

        self.so1_output /= 4.0;
        self.so2_output /= 4.0;

        self.so1_output *= self.so1_volume as f32 + 1.0;
        self.so2_output *= self.so2_volume as f32 + 1.0;
    }

    fn set_nr50(&mut self, v: u8) {
        self.nr50 = v;
        self.so1_volume = (v & 0x07) as u16;
        self.so2_volume = ((v >> 4) & 0x07) as u16;
    }

    fn get_nr52(&self) -> u8 {
        return (self.enable as u8) << 7 | (self.wave.enable as u8) << 2 | (self.square2.enable as u8) << 1;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff15..=0xff19 => self.square2.read(addr),
            0xff1a..=0xff1e|0xff30..=0xff3f => self.wave.read(addr),
            0xff24 => self.nr50,
            0xff25 => self.nr51,
            0xff26 => self.get_nr52(),
            0xff27..=0xff2f => mmu::NO_DATA,
            _ => mmu::NO_DATA,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff15..=0xff19 => self.square2.write(addr, value),
            0xff1a..=0xff1e|0xff30..=0xff3f => self.wave.write(addr, value),
            0xff24 => self.set_nr50(value),
            0xff25 => self.nr51 = value,
            0xff26 => self.enable = value & 0x80 != 0,
            0xff27..=0xff2f => (),
            _ => (),
        }
    }
}
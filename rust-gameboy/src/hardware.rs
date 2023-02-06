use std::{path::PathBuf, fs::File, io::{Read, Write}};

pub struct Hardware {
    path: PathBuf,
    save: PathBuf,
}

impl Hardware {
    pub fn new(path: PathBuf) -> Self {
        let mut save = path.clone();
        save.set_extension("sav");
        Hardware {
            path: path,
            save: save
        }
    }

    pub fn read_dot_gb(&self, buffer: &mut Vec<u8>) {
        let mut f = File::open(&self.path).expect("couldn't read file");
        f.read_to_end(buffer).expect("couldn't read file");
    }

    pub fn load_save(&self, buffer: &mut [u8]) {
        if !self.save.is_file() {
            return;
        }
        let mut f = File::open(&self.save).expect("couldn't read file");
        f.read(buffer).expect("couldn't read file");
    }

    pub fn save(&self, buffer: &[u8]) {
        let mut f = File::create(&self.save).expect("couldn't write file");
        f.write(buffer).expect("couldn't write file");
    }
}
extern crate minifb;
use minifb::{Window, WindowOptions};

pub struct Renderer {
    window: Window,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(height: usize, width: usize) -> Self {
        let mut window = Window::new("GameBoy", width, height, WindowOptions::default())
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });
        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
        Renderer {
            window: window,
            width: width,
            height: height,
        }
    }

    pub fn render(&mut self, buffer: &[u32]) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        self.window
            .update_with_buffer(&buffer, self.width, self.height)
            .unwrap();
    }

    const COLOR: [u32; 4] = [0, 90, 180, 255];

    pub fn render_u8(&mut self, buffer: &[u8]) {
        let mut b = vec![0; self.height * self.width];
        for y in 0..self.height {
            for x in 0..self.width {
                b[y * self.width + x] =
                    Renderer::COLOR[buffer[(y / 2) * (self.width / 2) + (x / 2)] as usize];
            }
        }
        self.window
            .update_with_buffer(&b, self.width, self.height)
            .unwrap();
    }
}

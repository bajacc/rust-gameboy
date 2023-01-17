extern crate minifb;
use minifb::{Window, WindowOptions};

pub struct Renderer {
    window: Window,
    width: usize,
    height: usize,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut window = Window::new(
            "Test - ESC to exit",
            width,
            height,
            WindowOptions::default(),
        )
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
}

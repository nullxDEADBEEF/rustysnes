use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 256;
const HEIGHT: usize = 224;

pub struct Snes {

}

impl Snes {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) {
        let window_options = WindowOptions {
            scale: minifb::Scale::X4,
            resize: true,
            ..WindowOptions::default()
        };

        let buffer: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

        let mut window = Window::new("RustySNES", WIDTH, HEIGHT, window_options)
            .unwrap_or_else(|e| panic!("{e}"));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        }
    }
}
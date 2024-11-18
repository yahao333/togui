use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Renderer {
    pixels: Pixels,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(
            window_size.width,
            window_size.height,
            window,
        );
        let pixels = Pixels::new(
            window_size.width,
            window_size.height,
            surface_texture,
        ).unwrap();

        Self { pixels }
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }

    pub fn render(&mut self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }
}
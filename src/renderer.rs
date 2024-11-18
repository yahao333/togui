use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Renderer {
    pixels: Pixels,
    width: u32,
    height: u32,
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

        Self { 
            pixels,
            width: window_size.width,
            height: window_size.height,
        }
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: [u8; 4]) {
        let frame = self.pixels.frame_mut();
        for dy in 0..height {
            for dx in 0..width {
                let px = x + dx as i32;
                let py = y + dy as i32;
                
                if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                    let idx = (py * self.width as i32 + px) as usize * 4;
                    frame[idx..idx + 4].copy_from_slice(&color);
                }
            }
        }
    }
    
    pub fn draw_pixel(&mut self, x: i32, y: i32, color: [u8; 4]) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = (y * self.width as i32 + x) as usize * 4;
            let frame = self.pixels.frame_mut();
            frame[idx..idx + 4].copy_from_slice(&color);
        }
    }

    pub fn render(&mut self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }
}
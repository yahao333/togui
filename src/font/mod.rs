// 基础的 8x8 点阵字体系统
pub struct Font {
    glyph_width: u32,
    glyph_height: u32,
    glyphs: &'static [[u8; 8]],
}

impl Font {
    pub fn default() -> Self {
        Self {
            glyph_width: 8,
            glyph_height: 8,
            glyphs: &BASIC_FONT,
        }
    }

    pub fn render_char(&self, renderer: &mut crate::renderer::Renderer, x: i32, y: i32, c: char, color: [u8; 4]) {
        if let Some(glyph) = self.get_glyph(c) {
            for (row, &bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if (bits >> (7 - col)) & 1 == 1 {
                        renderer.draw_pixel(
                            x + col as i32,
                            y + row as i32,
                            color
                        );
                    }
                }
            }
        }
    }

    fn get_glyph(&self, c: char) -> Option<&[u8; 8]> {
        if c.is_ascii() {
            Some(&self.glyphs[c as usize])
        } else {
            None
        }
    }
}

// 基础的 ASCII 字体数据
static BASIC_FONT: [[u8; 8]; 128] = [
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // 空格
    // ... 其他字符的数据将在后续添加
];
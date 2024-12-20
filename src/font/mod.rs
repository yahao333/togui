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
    // 0x00-0x1F: 控制字符 (32个)
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // NUL
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // SOH
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // STX
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ETX
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // EOT
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ENQ
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ACK
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // BEL
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // BS
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // TAB
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // LF
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // VT
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // FF
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // CR
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // SO
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // SI
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // DLE
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // DC1
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // DC2
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // DC3
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // DC4
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // NAK
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // SYN
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ETB
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // CAN
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // EM
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // SUB
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ESC
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // FS
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // GS
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // RS
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // US
    // ... 前30个控制字符都是空白
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // Space (32)
    [0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x00], // !
    [0x6C, 0x6C, 0x6C, 0x00, 0x00, 0x00, 0x00, 0x00], // "
    [0x6C, 0x6C, 0xFE, 0x6C, 0xFE, 0x6C, 0x6C, 0x00], // #
    [0x18, 0x7E, 0xC0, 0x7C, 0x06, 0xFC, 0x18, 0x00], // $
    [0x00, 0xC6, 0xCC, 0x18, 0x30, 0x66, 0xC6, 0x00], // %
    [0x38, 0x6C, 0x38, 0x76, 0xDC, 0xCC, 0x76, 0x00], // &
    [0x18, 0x18, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00], // '
    [0x0C, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0C, 0x00], // (
    [0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0x00], // )
    [0x00, 0x66, 0x3C, 0xFF, 0x3C, 0x66, 0x00, 0x00], // *
    [0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00], // +
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30], // ,
    [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00], // -
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00], // .
    [0x06, 0x0C, 0x18, 0x30, 0x60, 0xC0, 0x80, 0x00], // /
    // 数字 0-9
    [0x7C, 0xC6, 0xCE, 0xD6, 0xE6, 0xC6, 0x7C, 0x00], // 0
    [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00], // 1
    [0x7C, 0xC6, 0x06, 0x1C, 0x30, 0x66, 0xFE, 0x00], // 2
    [0x7C, 0xC6, 0x06, 0x3C, 0x06, 0xC6, 0x7C, 0x00], // 3
    [0x1C, 0x3C, 0x6C, 0xCC, 0xFE, 0x0C, 0x1E, 0x00], // 4
    [0xFE, 0xC0, 0xC0, 0xFC, 0x06, 0xC6, 0x7C, 0x00], // 5
    [0x38, 0x60, 0xC0, 0xFC, 0xC6, 0xC6, 0x7C, 0x00], // 6
    [0xFE, 0xC6, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00], // 7
    [0x7C, 0xC6, 0xC6, 0x7C, 0xC6, 0xC6, 0x7C, 0x00], // 8
    [0x7C, 0xC6, 0xC6, 0x7E, 0x06, 0x0C, 0x78, 0x00], // 9
    
    // 符号和大写字母
    [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00], // :
    [0x00, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x30], // ;
    [0x06, 0x0C, 0x18, 0x30, 0x18, 0x0C, 0x06, 0x00], // <
    [0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x00, 0x00], // =
    [0x60, 0x30, 0x18, 0x0C, 0x18, 0x30, 0x60, 0x00], // >
    [0x7C, 0xC6, 0x0C, 0x18, 0x18, 0x00, 0x18, 0x00], // ?
    [0x7C, 0xC6, 0xDE, 0xDE, 0xDE, 0xC0, 0x78, 0x00], // @

    // A-Z
    [0x38, 0x6C, 0xC6, 0xFE, 0xC6, 0xC6, 0xC6, 0x00], // A
    [0xFC, 0x66, 0x66, 0x7C, 0x66, 0x66, 0xFC, 0x00], // B
    [0x3C, 0x66, 0xC0, 0xC0, 0xC0, 0x66, 0x3C, 0x00], // C
    [0xF8, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0xF8, 0x00], // D
    [0xFE, 0x62, 0x68, 0x78, 0x68, 0x62, 0xFE, 0x00], // E
    [0xFE, 0x62, 0x68, 0x78, 0x68, 0x60, 0xF0, 0x00], // F
    [0x3C, 0x66, 0xC0, 0xC0, 0xCE, 0x66, 0x3E, 0x00], // G
    [0xC6, 0xC6, 0xC6, 0xFE, 0xC6, 0xC6, 0xC6, 0x00], // H
    [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00], // I
    [0x1E, 0x0C, 0x0C, 0x0C, 0xCC, 0xCC, 0x78, 0x00], // J
    [0xE6, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0xE6, 0x00], // K
    [0xF0, 0x60, 0x60, 0x60, 0x62, 0x66, 0xFE, 0x00], // L
    [0xC6, 0xEE, 0xFE, 0xFE, 0xD6, 0xC6, 0xC6, 0x00], // M
    [0xC6, 0xE6, 0xF6, 0xDE, 0xCE, 0xC6, 0xC6, 0x00], // N
    [0x38, 0x6C, 0xC6, 0xC6, 0xC6, 0x6C, 0x38, 0x00], // O
    [0xFC, 0x66, 0x66, 0x7C, 0x60, 0x60, 0xF0, 0x00], // P
    [0x7C, 0xC6, 0xC6, 0xC6, 0xD6, 0x7C, 0x0E, 0x00], // Q
    [0xFC, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0xE6, 0x00], // R
    [0x7C, 0xC6, 0x60, 0x38, 0x0C, 0xC6, 0x7C, 0x00], // S
    [0x7E, 0x7E, 0x5A, 0x18, 0x18, 0x18, 0x3C, 0x00], // T
    [0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0xC6, 0x7C, 0x00], // U
    [0xC6, 0xC6, 0xC6, 0xC6, 0x6C, 0x38, 0x10, 0x00], // V
    [0xC6, 0xC6, 0xD6, 0xFE, 0xFE, 0xEE, 0xC6, 0x00], // W
    [0xC6, 0x6C, 0x38, 0x38, 0x38, 0x6C, 0xC6, 0x00], // X
    [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x3C, 0x00], // Y
    [0xFE, 0xC6, 0x8C, 0x18, 0x32, 0x66, 0xFE, 0x00], // Z

    // 符号和小写字母
    [0x0C, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0C, 0x00], // [
    [0xC0, 0x60, 0x30, 0x18, 0x0C, 0x06, 0x02, 0x00], // \
    [0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0x00], // ]
    [0x10, 0x38, 0x6C, 0xC6, 0x00, 0x00, 0x00, 0x00], // ^
    [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF], // _
    [0x30, 0x18, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00], // `

    // a-z
    [0x00, 0x00, 0x78, 0x0C, 0x7C, 0xCC, 0x76, 0x00], // a
    [0xE0, 0x60, 0x60, 0x7C, 0x66, 0x66, 0xDC, 0x00], // b
    [0x00, 0x00, 0x78, 0xCC, 0xC0, 0xCC, 0x78, 0x00], // c
    [0x1C, 0x0C, 0x0C, 0x7C, 0xCC, 0xCC, 0x76, 0x00], // d
    [0x00, 0x00, 0x78, 0xCC, 0xFC, 0xC0, 0x78, 0x00], // e
    [0x38, 0x6C, 0x60, 0xF0, 0x60, 0x60, 0xF0, 0x00], // f
    [0x00, 0x00, 0x76, 0xCC, 0xCC, 0x7C, 0x0C, 0xF8], // g
    [0xE0, 0x60, 0x6C, 0x76, 0x66, 0x66, 0xE6, 0x00], // h
    [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x00], // i
    [0x06, 0x00, 0x06, 0x06, 0x06, 0x66, 0x66, 0x3C], // j
    [0xE0, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0xE6, 0x00], // k
    [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00], // l
    [0x00, 0x00, 0xCC, 0xFE, 0xFE, 0xD6, 0xC6, 0x00], // m
    [0x00, 0x00, 0xF8, 0xCC, 0xCC, 0xCC, 0xCC, 0x00], // n
    [0x00, 0x00, 0x78, 0xCC, 0xCC, 0xCC, 0x78, 0x00], // o
    [0x00, 0x00, 0xDC, 0x66, 0x66, 0x7C, 0x60, 0xF0], // p
    [0x00, 0x00, 0x76, 0xCC, 0xCC, 0x7C, 0x0C, 0x1E], // q
    [0x00, 0x00, 0xDC, 0x76, 0x66, 0x60, 0xF0, 0x00], // r
    [0x00, 0x00, 0x7C, 0xC0, 0x78, 0x0C, 0xF8, 0x00], // s
    [0x10, 0x30, 0x7C, 0x30, 0x30, 0x34, 0x18, 0x00], // t
    [0x00, 0x00, 0xCC, 0xCC, 0xCC, 0xCC, 0x76, 0x00], // u
    [0x00, 0x00, 0xCC, 0xCC, 0xCC, 0x78, 0x30, 0x00], // v
    [0x00, 0x00, 0xC6, 0xD6, 0xFE, 0xFE, 0x6C, 0x00], // w
    [0x00, 0x00, 0xC6, 0x6C, 0x38, 0x6C, 0xC6, 0x00], // x
    [0x00, 0x00, 0xCC, 0xCC, 0xCC, 0x7C, 0x0C, 0xF8], // y
    [0x00, 0x00, 0xFC, 0x98, 0x30, 0x64, 0xFC, 0x00], // z

    // 其他符号
    [0x1C, 0x30, 0x30, 0xE0, 0x30, 0x30, 0x1C, 0x00], // {
    [0x18, 0x18, 0x18, 0x00, 0x18, 0x18, 0x18, 0x00], // |
    [0xE0, 0x30, 0x30, 0x1C, 0x30, 0x30, 0xE0, 0x00], // }
    [0x76, 0xDC, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // ~
    [0x00, 0x10, 0x38, 0x6C, 0xC6, 0xC6, 0xFE, 0x00], // DEL    
];
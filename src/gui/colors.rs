use sdl2::pixels::Color;

pub const NTSC_PALETTE: [Color; 64] = [
    Color::RGB(84, 84, 84),    // 0x00
    Color::RGB(0, 30, 116),    // 0x01
    Color::RGB(8, 16, 144),    // 0x02
    Color::RGB(48, 0, 136),    // 0x03
    Color::RGB(68, 0, 100),    // 0x04
    Color::RGB(92, 0, 48),     // 0x05
    Color::RGB(84, 4, 0),      // 0x06
    Color::RGB(60, 24, 0),     // 0x07
    Color::RGB(32, 42, 0),     // 0x08
    Color::RGB(8, 58, 0),      // 0x09
    Color::RGB(0, 64, 0),      // 0x0A
    Color::RGB(0, 60, 0),      // 0x0B
    Color::RGB(0, 50, 60),     // 0x0C
    Color::RGB(0, 0, 0),       // 0x0D
    Color::RGB(0, 0, 0),       // 0x0E
    Color::RGB(0, 0, 0),       // 0x0F
    Color::RGB(152, 150, 152), // 0x10
    Color::RGB(8, 76, 196),    // 0x11
    Color::RGB(48, 50, 236),   // 0x12
    Color::RGB(92, 30, 228),   // 0x13
    Color::RGB(136, 20, 176),  // 0x14
    Color::RGB(160, 20, 100),  // 0x15
    Color::RGB(152, 34, 32),   // 0x16
    Color::RGB(120, 60, 0),    // 0x17
    Color::RGB(84, 90, 0),     // 0x18
    Color::RGB(40, 114, 0),    // 0x19
    Color::RGB(8, 124, 0),     // 0x1A
    Color::RGB(0, 118, 40),    // 0x1B
    Color::RGB(0, 102, 120),   // 0x1C
    Color::RGB(0, 0, 0),       // 0x1D
    Color::RGB(0, 0, 0),       // 0x1E
    Color::RGB(0, 0, 0),       // 0x1F
    Color::RGB(236, 238, 236), // 0x20
    Color::RGB(76, 154, 236),  // 0x21
    Color::RGB(120, 124, 236), // 0x22
    Color::RGB(176, 98, 236),  // 0x23
    Color::RGB(228, 84, 236),  // 0x24
    Color::RGB(236, 88, 180),  // 0x25
    Color::RGB(236, 106, 100), // 0x26
    Color::RGB(212, 136, 32),  // 0x27
    Color::RGB(160, 170, 0),   // 0x28
    Color::RGB(116, 196, 0),   // 0x29
    Color::RGB(76, 208, 32),   // 0x2A
    Color::RGB(56, 204, 108),  // 0x2B
    Color::RGB(56, 180, 204),  // 0x2C
    Color::RGB(60, 60, 60),    // 0x2D
    Color::RGB(0, 0, 0),       // 0x2E
    Color::RGB(0, 0, 0),       // 0x2F
    Color::RGB(236, 238, 236), // 0x30
    Color::RGB(168, 204, 236), // 0x31
    Color::RGB(188, 188, 236), // 0x32
    Color::RGB(212, 178, 236), // 0x33
    Color::RGB(236, 174, 236), // 0x34
    Color::RGB(236, 174, 212), // 0x35
    Color::RGB(236, 180, 176), // 0x36
    Color::RGB(228, 196, 144), // 0x37
    Color::RGB(204, 210, 120), // 0x38
    Color::RGB(180, 222, 120), // 0x39
    Color::RGB(168, 226, 144), // 0x3A
    Color::RGB(152, 226, 180), // 0x3B
    Color::RGB(160, 214, 228), // 0x3C
    Color::RGB(160, 162, 160), // 0x3D
    Color::RGB(0, 0, 0),       // 0x3E
    Color::RGB(0, 0, 0),       // 0x3F
];

pub fn get_rgb_from_idx(index: u8) -> Color {
    NTSC_PALETTE[index as usize]
}

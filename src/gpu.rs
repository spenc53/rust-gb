use crate::constants;

pub struct GPU {
    // tile_set: [Tile; 348],
    video_ram: [u8; 0xA000 - 0x8000],
    canvas_buffer: [u8; 1]
}

impl GPU {
    pub fn read_byte(&self, address: u16) -> u8 {
        return 0x0000;
    }
}
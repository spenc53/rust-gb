use crate::constants;
use crate::gpu::GPU;

pub struct MemoryBus {
    memory: [u8; 0xFFFF],
    gpu: GPU
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) ->  u8 {
        if (address >= constants::VIDEO_RAM_BEGIN && address < constants::VIDEO_RAM_END) {
            return self.gpu.read_byte(address);
        } else {
            return self.memory[address as usize];
        }
    }
}
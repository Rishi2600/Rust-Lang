pub const PAGE_SIZE: u32 = 4096; // 4KB
pub const PHYSICAL_FRAMES: usize = 16;

pub struct PhysicalMemory {
    pub data: Vec<u8>,
}

impl PhysicalMemory {
    pub fn new() -> Self {
        Self {
            data: vec![0; PHYSICAL_FRAMES * PAGE_SIZE as usize],
        }
    }

    pub fn read(&self, p_addr: u32) -> u8 {
        self.data[p_addr as usize]
    }

    pub fn write(&mut self, p_addr: u32, val: u8) {
        self.data[p_addr as usize] = val;
    }
}
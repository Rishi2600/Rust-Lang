use crate::hardware::{PhysicalMemory, PAGE_SIZE};
use crate::errors::MemoryError;

#[derive(Clone, Debug, PartialEq)]
pub enum PageState {
    Unallocated,
    InRam { frame_id: usize },
    OnDisk,
}

pub struct MMU {
    pub page_table: Vec<PageState>,
    pub ram: PhysicalMemory,
}

impl MMU {
    pub fn new(total_pages: usize) -> Self {
        Self {
            page_table: vec![PageState::Unallocated; total_pages],
            ram: PhysicalMemory::new(),
        }
    }

    // The "Superpower" Bit-Masking Logic
    pub fn translate(&self, v_addr: u32) -> Result<u32, MemoryError> {
        // Offset is the last 12 bits (0xFFF)
        let offset = v_addr & 0xFFF; 
        // Page Number is everything before the last 12 bits
        let page_num = (v_addr >> 12) as usize;

        match self.page_table.get(page_num) {
            Some(PageState::InRam { frame_id }) => {
                let phys_base = (*frame_id as u32) << 12; // Shift back to align
                Ok(phys_base | offset) // Combine base and offset with OR
            }
            Some(PageState::OnDisk) => Err(MemoryError::PageFault(v_addr)),
            _ => Err(MemoryError::SegmentationFault(v_addr)),
        }
    }
}
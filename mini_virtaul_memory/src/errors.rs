#[derive(Debug)]
pub enum MemoryError {
    SegmentationFault(u32), // Accessing unallocated memory
    PageFault(u32),         // Memory is on disk, not RAM
    InvalidAddress,         // Address out of bounds
}
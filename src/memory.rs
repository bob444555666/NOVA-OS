//! Allocation-free physical page-frame allocator primitives.
//!
//! The boot path must populate this allocator from the bootloader memory map
//! and reserve kernel/image/bootloader regions before calling allocate_frame.
//! This module intentionally does not guess physical memory ranges.

pub const PAGE_SIZE: u64 = 4096;

/// Bitmap-backed frame allocator over a caller-owned bitmap. One bit represents
/// one physical 4 KiB frame. A set bit means allocated/reserved.
pub struct FrameAllocator<'a> {
    bitmap: &'a mut [u8],
    frame_count: usize,
    next_hint: usize,
}

impl<'a> FrameAllocator<'a> {
    /// Creates an allocator for frame_count frames. All frames begin reserved.
    /// Call mark_usable only for ranges confirmed usable by the bootloader.
    pub fn new(bitmap: &'a mut [u8], frame_count: usize) -> Self {
        let needed = frame_count.saturating_add(7) / 8;
        let usable_count = core::cmp::min(bitmap.len(), needed);
        let mut i = 0;
        while i < usable_count {
            bitmap[i] = 0xFF;
            i += 1;
        }
        let bounded_frame_count = core::cmp::min(frame_count, bitmap.len().saturating_mul(8));
        Self { bitmap, frame_count: bounded_frame_count, next_hint: 0 }
    }

    /// Marks a frame-index range usable. This must only be used for RAM ranges
    /// explicitly reported as usable by the boot protocol.
    pub fn mark_usable(&mut self, start_frame: usize, count: usize) {
        let end = core::cmp::min(self.frame_count, start_frame.saturating_add(count));
        let mut frame = start_frame;
        while frame < end {
            self.set_allocated(frame, false);
            frame += 1;
        }
        self.next_hint = core::cmp::min(self.next_hint, start_frame);
    }

    /// Reserves a frame-index range (kernel, modules, framebuffer, firmware,
    /// boot structures, MMIO, or anything else that must not be allocated).
    pub fn reserve(&mut self, start_frame: usize, count: usize) {
        let end = core::cmp::min(self.frame_count, start_frame.saturating_add(count));
        let mut frame = start_frame;
        while frame < end {
            self.set_allocated(frame, true);
            frame += 1;
        }
    }

    /// Allocates one frame and returns its physical byte address.
    pub fn allocate_frame(&mut self) -> Option<u64> {
        if self.frame_count == 0 { return None; }
        let mut checked = 0usize;
        let mut frame = self.next_hint;
        while checked < self.frame_count {
            if !self.is_allocated(frame) {
                self.set_allocated(frame, true);
                self.next_hint = (frame + 1) % self.frame_count;
                return Some((frame as u64) * PAGE_SIZE);
            }
            frame = (frame + 1) % self.frame_count;
            checked += 1;
        }
        None
    }

    /// Releases a frame previously allocated by this allocator.
    /// Caller must ensure the frame is not still referenced and is in RAM.
    pub fn free_frame(&mut self, physical_address: u64) -> Result<(), FrameError> {
        if physical_address % PAGE_SIZE != 0 { return Err(FrameError::Unaligned); }
        let frame_u64 = physical_address / PAGE_SIZE;
        if frame_u64 >= self.frame_count as u64 { return Err(FrameError::OutOfRange); }
        let frame = frame_u64 as usize;
        if !self.is_allocated(frame) { return Err(FrameError::AlreadyFree); }
        self.set_allocated(frame, false);
        self.next_hint = core::cmp::min(self.next_hint, frame);
        Ok(())
    }

    pub fn total_frames(&self) -> usize { self.frame_count }

    fn is_allocated(&self, frame: usize) -> bool {
        (self.bitmap[frame / 8] & (1 << (frame % 8))) != 0
    }

    fn set_allocated(&mut self, frame: usize, allocated: bool) {
        let mask = 1u8 << (frame % 8);
        if allocated { self.bitmap[frame / 8] |= mask; }
        else { self.bitmap[frame / 8] &= !mask; }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameError {
    Unaligned,
    OutOfRange,
    AlreadyFree,
}

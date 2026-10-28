//! Limine memory-map integration for NOVA's physical frame allocator.
//!
//! Only entries explicitly tagged USABLE by Limine are released into the
//! allocator. All other memory (kernel/modules, firmware, framebuffer,
//! bootloader structures, reserved ranges and MMIO) remains unavailable.

use crate::memory::{FrameAllocator, PAGE_SIZE};
use core::ffi::c_void;

const COMMON_MAGIC_0: u64 = 0xc7b1_dd30_df4c_8b88;
const COMMON_MAGIC_1: u64 = 0x0a82_e883_a194_f07b;
const MEMMAP_REQUEST_ID_0: u64 = 0x67cf_3d9d_378a_806f;
const MEMMAP_REQUEST_ID_1: u64 = 0xe304_acdf_c50c_3c62;
const MEMMAP_USABLE: u64 = 0;
const MAX_MEMMAP_ENTRIES: usize = 4096;

// 1 MiB covers 8,388,608 4-KiB frames (32 GiB of physical address space).
// This lives in the kernel image's BSS, so Limine never reports it as usable.
const BITMAP_BYTES: usize = 1024 * 1024;
const FRAME_CAPACITY: usize = BITMAP_BYTES * 8;

#[repr(C)]
struct RawMemmapRequest {
    id: [u64; 4],
    revision: u64,
    response: *mut RawMemmapResponse,
}

#[repr(C)]
struct RawMemmapResponse {
    revision: u64,
    entry_count: u64,
    entries: *const *const RawMemmapEntry,
}

#[repr(C)]
struct RawMemmapEntry {
    base: u64,
    length: u64,
    kind: u64,
}

#[used]
#[link_section = ".requests"]
static mut MEMMAP_REQUEST: RawMemmapRequest = RawMemmapRequest {
    id: [COMMON_MAGIC_0, COMMON_MAGIC_1, MEMMAP_REQUEST_ID_0, MEMMAP_REQUEST_ID_1],
    revision: 0,
    response: core::ptr::null_mut(),
};

static mut FRAME_BITMAP: [u8; BITMAP_BYTES] = [0; BITMAP_BYTES];
static mut FRAME_ALLOCATOR: Option<FrameAllocator<'static>> = None;

#[derive(Clone, Copy)]
pub struct MemoryStats {
    pub map_entries: usize,
    pub usable_ranges: usize,
    pub usable_frames: usize,
    pub allocator_frames: usize,
    pub truncated: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InitError {
    MissingResponse,
    InvalidMap,
}

/// Build the live frame allocator from Limine's actual memory map.
/// This function must be called once, before any physical frame allocation.
pub fn initialize() -> Result<MemoryStats, InitError> {
    unsafe {
        let response = core::ptr::read_volatile(
            core::ptr::addr_of!(MEMMAP_REQUEST.response)
        );
        if response.is_null() {
            return Err(InitError::MissingResponse);
        }

        let response = &*response;
        let entry_count = usize::try_from(response.entry_count)
            .map_err(|_| InitError::InvalidMap)?;
        if entry_count == 0 || entry_count > MAX_MEMMAP_ENTRIES || response.entries.is_null() {
            return Err(InitError::InvalidMap);
        }

        let bitmap: &'static mut [u8] = &mut *core::ptr::addr_of_mut!(FRAME_BITMAP);
        let mut allocator = FrameAllocator::new(bitmap, FRAME_CAPACITY);
        let mut usable_ranges = 0usize;
        let mut usable_frames = 0usize;
        let mut truncated = false;

        let mut index = 0usize;
        while index < entry_count {
            let entry_ptr = core::ptr::read(response.entries.add(index));
            if entry_ptr.is_null() {
                return Err(InitError::InvalidMap);
            }
            let entry = &*entry_ptr;
            if entry.kind == MEMMAP_USABLE && entry.length >= PAGE_SIZE {
                // Limine guarantees usable ranges are page-aligned. Align
                // defensively anyway, and avoid overflow in base + length.
                let start = entry.base.saturating_add(PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
                let end = entry.base.saturating_add(entry.length) & !(PAGE_SIZE - 1);
                if end > start {
                    let first_frame = start / PAGE_SIZE;
                    let last_frame = end / PAGE_SIZE;
                    if first_frame < FRAME_CAPACITY as u64 {
                        let bounded_end = core::cmp::min(last_frame, FRAME_CAPACITY as u64);
                        if bounded_end > first_frame {
                            let first = first_frame as usize;
                            let count = (bounded_end - first_frame) as usize;
                            allocator.mark_usable(first, count);
                            usable_ranges += 1;
                            usable_frames = usable_frames.saturating_add(count);
                        }
                    }
                    if last_frame > FRAME_CAPACITY as u64 {
                        truncated = true;
                    }
                }
            }
            index += 1;
        }

        // Never hand out the null page, even if a firmware map lists it usable.
        allocator.reserve(0, 1);
        let allocator_frames = allocator.total_frames();
        FRAME_ALLOCATOR = Some(allocator);

        Ok(MemoryStats {
            map_entries: entry_count,
            usable_ranges,
            usable_frames,
            allocator_frames,
            truncated,
        })
    }
}

/// Allocate one 4-KiB physical frame from the boot map-derived allocator.
pub fn allocate_frame() -> Option<u64> {
    unsafe { FRAME_ALLOCATOR.as_mut()?.allocate_frame() }
}

/// Free a previously allocated physical frame.
pub fn free_frame(address: u64) -> Result<(), crate::memory::FrameError> {
    unsafe { FRAME_ALLOCATOR.as_mut().ok_or(crate::memory::FrameError::OutOfRange)?.free_frame(address) }
}


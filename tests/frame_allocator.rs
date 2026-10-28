// Host-side tests for the allocator primitives shared with the bare-metal kernel.
#[path = "../src/memory.rs"]
mod memory;

use memory::{FrameAllocator, FrameError, PAGE_SIZE};

#[test]
fn frames_start_reserved_and_only_usable_frames_allocate() {
    let mut bitmap = [0u8; 2];
    let mut allocator = FrameAllocator::new(&mut bitmap, 10);
    assert_eq!(allocator.total_frames(), 10);
    assert_eq!(allocator.allocate_frame(), None);

    allocator.mark_usable(2, 3);
    assert_eq!(allocator.allocate_frame(), Some(2 * PAGE_SIZE));
    assert_eq!(allocator.allocate_frame(), Some(3 * PAGE_SIZE));
    assert_eq!(allocator.allocate_frame(), Some(4 * PAGE_SIZE));
    assert_eq!(allocator.allocate_frame(), None);
}

#[test]
fn free_and_reallocate_frame() {
    let mut bitmap = [0u8; 1];
    let mut allocator = FrameAllocator::new(&mut bitmap, 8);
    allocator.mark_usable(1, 2);
    let first = allocator.allocate_frame().unwrap();
    assert_eq!(first, PAGE_SIZE);
    allocator.free_frame(first).unwrap();
    assert_eq!(allocator.allocate_frame(), Some(first));
}

#[test]
fn invalid_free_addresses_are_rejected() {
    let mut bitmap = [0u8; 1];
    let mut allocator = FrameAllocator::new(&mut bitmap, 8);
    allocator.mark_usable(0, 1);
    assert_eq!(allocator.free_frame(1), Err(FrameError::Unaligned));
    assert_eq!(allocator.free_frame(8 * PAGE_SIZE), Err(FrameError::OutOfRange));
    assert_eq!(allocator.free_frame(0), Err(FrameError::AlreadyFree));
}

#[test]
fn bitmap_capacity_bounds_frame_count() {
    let mut bitmap = [0u8; 1];
    let allocator = FrameAllocator::new(&mut bitmap, usize::MAX);
    assert_eq!(allocator.total_frames(), 8);
}

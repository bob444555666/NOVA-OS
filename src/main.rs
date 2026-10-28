#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

#[used]
#[link_section = ".requests_start_marker"]
static REQUESTS_START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[link_section = ".requests_end_marker"]
static REQUESTS_END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = response.framebuffers().next() {
            let width = framebuffer.width() as usize;
            let height = framebuffer.height() as usize;
            let pitch = framebuffer.pitch() as usize;
            let bpp = framebuffer.bpp() as usize;
            let bytes_per_pixel = bpp / 8;
            if bytes_per_pixel >= 3 && width > 0 && height > 0 {
                fill_screen(framebuffer.addr(), width, height, pitch, bytes_per_pixel);
            }
        }
    }
    loop { core::hint::spin_loop(); }
}

fn fill_screen(buffer: *mut u8, width: usize, height: usize, pitch: usize, bpp: usize) {
    for y in 0..height {
        for x in 0..width {
            let offset = y * pitch + x * bpp;
            unsafe {
                buffer.add(offset).write_volatile(12);
                buffer.add(offset + 1).write_volatile(16);
                buffer.add(offset + 2).write_volatile(28);
            }
        }
    }

    let radius = core::cmp::min(width, height) / 10;
    if radius < 8 { return; }
    let cx = width / 2;
    let cy = height / 2;
    for dy in 0..(radius * 2) {
        for dx in 0..(radius * 2) {
            let x = cx - radius + dx;
            let y = cy - radius + dy;
            if x >= width || y >= height { continue; }
            let ax = if dx > radius { dx - radius } else { radius - dx };
            let ay = if dy > radius { dy - radius } else { radius - dy };
            if ax + ay <= radius {
                let offset = y * pitch + x * bpp;
                unsafe {
                    buffer.add(offset).write_volatile(70);
                    buffer.add(offset + 1).write_volatile(150);
                    buffer.add(offset + 2).write_volatile(255);
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { core::hint::spin_loop(); }
}

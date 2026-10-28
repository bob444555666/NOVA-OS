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
    serial_init();
    serial_write_str("\r\nNOVA OS: kernel entered.\r\n");

    if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = response.framebuffers().next() {
            let width = framebuffer.width() as usize;
            let height = framebuffer.height() as usize;
            let pitch = framebuffer.pitch() as usize;
            let bpp = framebuffer.bpp() as usize;
            let bytes_per_pixel = bpp / 8;

            if bytes_per_pixel >= 3 && width > 0 && height > 0 {
                serial_write_str("NOVA OS: framebuffer found; drawing splash screen.\r\n");
                fill_screen(framebuffer.addr(), width, height, pitch, bytes_per_pixel);
            } else {
                serial_write_str("NOVA OS: framebuffer format is unsupported.\r\n");
            }
        } else {
            serial_write_str("NOVA OS: no framebuffer was provided.\r\n");
        }
    } else {
        serial_write_str("NOVA OS: bootloader did not return a framebuffer.\r\n");
    }

    serial_write_str("NOVA OS: early boot complete; CPU halted in idle loop.\r\n");
    loop { core::hint::spin_loop(); }
}

fn fill_screen(buffer: *mut u8, width: usize, height: usize, pitch: usize, bpp: usize) {
    // Clear the framebuffer to a dark navy background.
    for y in 0..height {
        for x in 0..width {
            let offset = y * pitch + x * bpp;
            unsafe {
                buffer.add(offset).write_volatile(12);
                buffer.add(offset + 1).write_volatile(16);
                buffer.add(offset + 2).write_volatile(28);
                if bpp >= 4 {
                    buffer.add(offset + 3).write_volatile(0);
                }
            }
        }
    }

    // Draw a blue diamond as the first NOVA visual mark.
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
                    if bpp >= 4 {
                        buffer.add(offset + 3).write_volatile(0);
                    }
                }
            }
        }
    }
}

fn serial_init() {
    // Configure COM1 (0x3F8) for 8-N-1. Output remains non-blocking so the
    // kernel won't hang on machines without a connected serial console.
    unsafe {
        outb(0x3F9, 0x00);
        outb(0x3FB, 0x80);
        outb(0x3F8, 0x03);
        outb(0x3F9, 0x00);
        outb(0x3FB, 0x03);
        outb(0x3FA, 0xC7);
        outb(0x3FC, 0x0B);
    }
}

fn serial_write_str(value: &str) {
    for byte in value.bytes() {
        unsafe { outb(0x3F8, byte); }
    }
}

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_write_str("\r\nNOVA OS: kernel panic. Halting.\r\n");
    loop { core::hint::spin_loop(); }
}

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
        if let Some(fb) = response.framebuffers().next() {
            let w = fb.width() as usize;
            let h = fb.height() as usize;
            let pitch = fb.pitch() as usize;
            let bpp = (fb.bpp() / 8) as usize;
            if w > 0 && h > 0 && bpp >= 3 {
                serial_write_str("NOVA OS: framebuffer found; rendering NOVA desktop.\r\n");
                draw_desktop(fb.addr(), w, h, pitch, bpp);
                serial_write_str("NOVA OS: desktop rendered; input and applications are not implemented yet.\r\n");
            } else {
                serial_write_str("NOVA OS: unsupported framebuffer format.\r\n");
            }
        } else {
            serial_write_str("NOVA OS: no framebuffer was provided.\r\n");
        }
    } else {
        serial_write_str("NOVA OS: bootloader did not return a framebuffer.\r\n");
    }

    serial_shell();
}

#[derive(Clone, Copy)]
struct Color(u8, u8, u8);

const BG: Color = Color(10, 15, 29);
const PANEL: Color = Color(19, 27, 47);
const CARD: Color = Color(27, 38, 63);
const WHITE: Color = Color(239, 245, 255);
const MUTED: Color = Color(145, 164, 194);
const CYAN: Color = Color(66, 214, 255);
const BLUE: Color = Color(70, 125, 255);
const ORANGE: Color = Color(255, 151, 67);
const GREEN: Color = Color(82, 225, 164);
const PURPLE: Color = Color(174, 125, 255);

fn draw_desktop(buf: *mut u8, w: usize, h: usize, pitch: usize, bpp: usize) {
    rect(buf, w, h, pitch, bpp, 0, 0, w, h, BG);

    // Ambient light pools and a subtle grid create a layered, modern desktop.
    rect(buf, w, h, pitch, bpp, 0, 0, w, h / 3, Color(13, 22, 43));
    let mut gx = 0usize;
    while gx < w { line_v(buf, w, h, pitch, bpp, gx, 0, h, Color(17, 26, 46)); gx += 48; }
    let mut gy = 0usize;
    while gy < h { line_h(buf, w, h, pitch, bpp, 0, w, gy, Color(17, 26, 46)); gy += 48; }

    // Top brand bar.
    rect(buf, w, h, pitch, bpp, 0, 0, w, 58, PANEL);
    rect(buf, w, h, pitch, bpp, 0, 57, w, 2, CYAN);
    draw_logo(buf, w, h, pitch, bpp, 22, 13, 30);
    text(buf, w, h, pitch, bpp, 62, 15, "NOVA", WHITE, 2);
    text(buf, w, h, pitch, bpp, 63, 36, "OPERATING SYSTEM", MUTED, 1);
    let right_label = if w > 600 { w - 178 } else { w.saturating_sub(130) };
    text(buf, w, h, pitch, bpp, right_label, 24, "PREVIEW BUILD", CYAN, 1);

    // Welcome hero.
    let margin = if w < 640 { 18 } else { 42 };
    text(buf, w, h, pitch, bpp, margin, 82, "WELCOME TO NOVA", WHITE, 2);
    text(buf, w, h, pitch, bpp, margin, 106, "A fresh space for what's next.", MUTED, 1);

    // Status card.
    let card_y = 137usize;
    let card_h = if h > 520 { 88 } else { 72 };
    rect(buf, w, h, pitch, bpp, margin, card_y, w.saturating_sub(margin * 2), card_h, CARD);
    rect(buf, w, h, pitch, bpp, margin, card_y, 4, card_h, CYAN);
    draw_logo(buf, w, h, pitch, bpp, margin + 18, card_y + 20, 42);
    text(buf, w, h, pitch, bpp, margin + 75, card_y + 16, "YOUR DESKTOP IS READY", WHITE, 1);
    text(buf, w, h, pitch, bpp, margin + 75, card_y + 36, "Core system started successfully.", MUTED, 1);
    text(buf, w, h, pitch, bpp, margin + 75, card_y + 54, "Apps and input are still in development.", ORANGE, 1);

    // App launcher grid.
    let app_y = card_y + card_h + 26;
    text(buf, w, h, pitch, bpp, margin, app_y, "APPS", WHITE, 1);
    text(buf, w, h, pitch, bpp, margin + 50, app_y, "EARLY ACCESS", MUTED, 1);

    let grid_y = app_y + 24;
    let available = w.saturating_sub(margin * 2);
    let gap = 12usize;
    let tile_w = core::cmp::min(available.saturating_sub(gap * 3) / 4, 150);
    let tile_h = 100usize;
    let labels = ["FILES", "SETTINGS", "BROWSER", "APP CENTER"];
    let subs = ["Explore storage", "Personalize NOVA", "Not installed", "More apps soon"];
    let colors = [CYAN, PURPLE, GREEN, ORANGE];
    let mut i = 0usize;
    while i < 4 {
        let x = margin + i * (tile_w + gap);
        if x + tile_w <= w {
            rect(buf, w, h, pitch, bpp, x, grid_y, tile_w, tile_h, CARD);
            rect(buf, w, h, pitch, bpp, x, grid_y, tile_w, 3, colors[i]);
            draw_app_icon(buf, w, h, pitch, bpp, x + 14, grid_y + 14, i, colors[i]);
            text(buf, w, h, pitch, bpp, x + 14, grid_y + 58, labels[i], WHITE, 1);
            text(buf, w, h, pitch, bpp, x + 14, grid_y + 77, subs[i], MUTED, 1);
        }
        i += 1;
    }

    // Bottom taskbar.
    let bar_h = 48usize;
    let bar_y = h.saturating_sub(bar_h);
    rect(buf, w, h, pitch, bpp, 0, bar_y, w, bar_h, PANEL);
    rect(buf, w, h, pitch, bpp, 0, bar_y, w, 2, Color(44, 61, 89));
    draw_logo(buf, w, h, pitch, bpp, 16, bar_y + 10, 27);
    text(buf, w, h, pitch, bpp, 53, bar_y + 19, "NOVA", WHITE, 1);
    text(buf, w, h, pitch, bpp, 112, bar_y + 19, "DESKTOP PREVIEW", MUTED, 1);
    if w > 420 {
        text(buf, w, h, pitch, bpp, w - 143, bar_y + 19, "SYSTEM ONLINE", GREEN, 1);
    }

    // Small explicit note prevents the prototype from pretending a browser works.
    if h > 430 {
        text(buf, w, h, pitch, bpp, margin, grid_y + tile_h + 22,
             "NOTE: Browser, mouse, keyboard and app launching are not available yet.", MUTED, 1);
    }
}

fn draw_logo(buf: *mut u8, w: usize, h: usize, pitch: usize, bpp: usize, x: usize, y: usize, size: usize) {
    if size < 8 { return; }
    let half = size / 2;
    let mut dy = 0usize;
    while dy < size {
        let mut dx = 0usize;
        while dx < size {
            let ax = if dx > half { dx - half } else { half - dx };
            let ay = if dy > half { dy - half } else { half - dy };
            if ax + ay <= half {
                let c = if dy < half { CYAN } else { BLUE };
                pixel(buf, w, h, pitch, bpp, x + dx, y + dy, c);
            }
            dx += 1;
        }
        dy += 1;
    }
    // A small orange core makes the mark distinctive.
    rect(buf, w, h, pitch, bpp, x + half - 2, y + half - 2, 5, 5, ORANGE);
}

fn draw_app_icon(buf: *mut u8, w: usize, h: usize, pitch: usize, bpp: usize, x: usize, y: usize, which: usize, color: Color) {
    rect(buf, w, h, pitch, bpp, x, y, 31, 31, Color(15, 23, 41));
    match which {
        0 => { // folder
            rect(buf, w, h, pitch, bpp, x + 5, y + 11, 21, 14, color);
            rect(buf, w, h, pitch, bpp, x + 6, y + 7, 10, 6, color);
        }
        1 => { // settings gear-like cross
            rect(buf, w, h, pitch, bpp, x + 12, y + 5, 7, 21, color);
            rect(buf, w, h, pitch, bpp, x + 5, y + 12, 21, 7, color);
            rect(buf, w, h, pitch, bpp, x + 9, y + 9, 13, 13, color);
            rect(buf, w, h, pitch, bpp, x + 13, y + 13, 5, 5, Color(15, 23, 41));
        }
        2 => { // browser window
            rect(buf, w, h, pitch, bpp, x + 4, y + 5, 23, 21, color);
            rect(buf, w, h, pitch, bpp, x + 6, y + 11, 19, 13, Color(15, 23, 41));
            rect(buf, w, h, pitch, bpp, x + 7, y + 7, 3, 2, ORANGE);
            rect(buf, w, h, pitch, bpp, x + 12, y + 7, 3, 2, WHITE);
        }
        _ => { // app store box
            rect(buf, w, h, pitch, bpp, x + 5, y + 5, 21, 21, color);
            rect(buf, w, h, pitch, bpp, x + 9, y + 9, 13, 13, Color(15, 23, 41));
            rect(buf, w, h, pitch, bpp, x + 14, y + 11, 3, 9, color);
            rect(buf, w, h, pitch, bpp, x + 11, y + 14, 9, 3, color);
        }
    }
}

fn pixel(buf: *mut u8, w: usize, h: usize, pitch: usize, bpp: usize, x: usize, y: usize, c: Color) {
    if x >= w || y >= h { return; }
    let off = y * pitch + x * bpp;
    unsafe {
        // Limine's framebuffer on the tested targets uses BGR byte order.
        buf.add(off).write_volatile(c.2);
        buf.add(off + 1).write_volatile(c.1);
        buf.add(off + 2).write_volatile(c.0);
        if bpp >= 4 { buf.add(off + 3).write_volatile(0); }
    }
}

fn rect(buf: *mut u8, w: usize, h: usize, pitch: usize, bpp: usize, x: usize, y: usize, rw: usize, rh: usize, c: Color) {
    let x_end = core::cmp::min(w, x.saturating_add(rw));
    let y_end = core::cmp::min(h, y.saturating_add(rh));
    let mut yy = y;
    while yy < y_end {
        let mut xx = x;
        while xx < x_end { pixel(buf, w, h, pitch, bpp, xx, yy, c); xx += 1; }
        yy += 1;
    }
}
fn line_h(buf: *mut u8, w: usize, h: usize, pitch: usize, bpp: usize, x: usize, end: usize, y: usize, c: Color) {
    let mut xx = x;
    while xx < core::cmp::min(w, end) { pixel(buf, w, h, pitch, bpp, xx, y, c); xx += 1; }
}
fn line_v(buf: *mut u8, w: usize, h: usize, pitch: usize, bpp: usize, x: usize, y: usize, end: usize, c: Color) {
    let mut yy = y;
    while yy < core::cmp::min(h, end) { pixel(buf, w, h, pitch, bpp, x, yy, c); yy += 1; }
}

// Compact 5x7 uppercase bitmap font. Text is drawn directly into the framebuffer.
fn glyph(ch: u8) -> [u8; 7] {
    match ch {
        b'A' => [14,17,17,31,17,17,17], b'B' => [30,17,17,30,17,17,30],
        b'C' => [14,17,16,16,16,17,14], b'D' => [30,17,17,17,17,17,30],
        b'E' => [31,16,16,30,16,16,31], b'F' => [31,16,16,30,16,16,16],
        b'G' => [14,17,16,23,17,17,15], b'H' => [17,17,17,31,17,17,17],
        b'I' => [14,4,4,4,4,4,14], b'J' => [7,2,2,2,18,18,12],
        b'K' => [17,18,20,24,20,18,17], b'L' => [16,16,16,16,16,16,31],
        b'M' => [17,27,21,21,17,17,17], b'N' => [17,25,21,19,17,17,17],
        b'O' => [14,17,17,17,17,17,14], b'P' => [30,17,17,30,16,16,16],
        b'Q' => [14,17,17,17,21,18,13], b'R' => [30,17,17,30,20,18,17],
        b'S' => [15,16,16,14,1,1,30], b'T' => [31,4,4,4,4,4,4],
        b'U' => [17,17,17,17,17,17,14], b'V' => [17,17,17,17,17,10,4],
        b'W' => [17,17,17,21,21,21,10], b'X' => [17,17,10,4,10,17,17],
        b'Y' => [17,17,10,4,4,4,4], b'Z' => [31,1,2,4,8,16,31],
        b'0' => [14,17,19,21,25,17,14], b'1' => [4,12,4,4,4,4,14],
        b'2' => [14,17,1,2,4,8,31], b'3' => [30,1,1,14,1,1,30],
        b'4' => [2,6,10,18,31,2,2], b'5' => [31,16,16,30,1,1,30],
        b'6' => [14,16,16,30,17,17,14], b'7' => [31,1,2,4,8,8,8],
        b'8' => [14,17,17,14,17,17,14], b'9' => [14,17,17,15,1,1,14],
        b'.' => [0,0,0,0,0,12,12], b',' => [0,0,0,0,0,12,8],
        b':' => [0,12,12,0,12,12,0], b'-' => [0,0,0,31,0,0,0],
        b'\'' => [12,12,4,0,0,0,0], b'!' => [4,4,4,4,4,0,4],
        b'?' => [14,17,1,2,4,0,4], b'/' => [1,2,2,4,8,8,16],
        b'+' => [0,4,4,31,4,4,0], b' ' => [0;7],
        _ => [31,17,5,9,4,0,4],
    }
}
fn text(buf: *mut u8, w: usize, h: usize, pitch: usize, bpp: usize, x: usize, y: usize, s: &str, c: Color, scale: usize) {
    let mut cx = x;
    for byte in s.bytes() {
        let g = glyph(byte.to_ascii_uppercase());
        let mut row = 0usize;
        while row < 7 {
            let mut col = 0usize;
            while col < 5 {
                if (g[row] & (1 << (4 - col))) != 0 {
                    rect(buf, w, h, pitch, bpp, cx + col * scale, y + row * scale, scale, scale, c);
                }
                col += 1;
            }
            row += 1;
        }
        cx = cx.saturating_add(6 * scale);
        if cx >= w { break; }
    }
}

fn serial_init() {
    unsafe {
        outb(0x3F9, 0x00); outb(0x3FB, 0x80); outb(0x3F8, 0x03);
        outb(0x3F9, 0x00); outb(0x3FB, 0x03); outb(0x3FA, 0xC7); outb(0x3FC, 0x0B);
    }
}
fn serial_write_str(value: &str) { for byte in value.bytes() { serial_write_byte(byte); } }

fn serial_write_byte(byte: u8) {
    unsafe {
        // Wait until the UART transmit holding register is empty.
        while (inb(0x3FD) & 0x20) == 0 { core::hint::spin_loop(); }
        outb(0x3F8, byte);
    }
}

fn serial_read_byte() -> Option<u8> {
    unsafe {
        if (inb(0x3FD) & 0x01) != 0 { Some(inb(0x3F8)) } else { None }
    }
}

// A deliberately small, allocation-free kernel monitor. This is a real command
// loop over COM1 for debugging while the graphical input stack is developed.
fn serial_shell() -> ! {
    let mut line = [0u8; 80];
    let mut len = 0usize;
    serial_write_str("\r\nNOVA OS: interactive serial shell ready. Type help.\r\nnova> ");
    loop {
        if let Some(byte) = serial_read_byte() {
            match byte {
                b'\r' | b'\n' => {
                    serial_write_str("\r\n");
                    run_command(&line[..len]);
                    len = 0;
                    serial_write_str("nova> ");
                }
                8 | 127 => {
                    if len > 0 {
                        len -= 1;
                        serial_write_str("\x08 \x08");
                    }
                }
                32..=126 if len < line.len() => {
                    line[len] = byte;
                    len += 1;
                    serial_write_byte(byte);
                }
                _ => {}
            }
        } else {
            core::hint::spin_loop();
        }
    }
}

fn run_command(command: &[u8]) {
    match command {
        b"" => {}
        b"help" => serial_write_str("Commands: help, about, version, clear, halt\r\n"),
        b"about" => serial_write_str("NOVA is an experimental independent x86-64 kernel. GUI, drivers, storage and networking are not complete yet.\r\n"),
        b"version" => serial_write_str("NOVA kernel 0.1.0 (early development)\r\n"),
        b"clear" => serial_write_str("\x1B[2J\x1B[H"),
        b"halt" => {
            serial_write_str("NOVA: CPU halted. Reset the virtual machine to continue.\r\n");
            loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); } }
        }
        _ => serial_write_str("Unknown command. Type help.\r\n"),
    }
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack, preserves_flags));
    value
}

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_write_str("\r\nNOVA OS: kernel panic. Halting.\r\n");
    loop { core::hint::spin_loop(); }
}

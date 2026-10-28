//! Minimal x86-64 IDT: every vector enters a fail-stop exception handler.
//! This establishes a kernel-owned IDT before enabling interrupts. IRQ routing,
//! per-vector diagnostics, recovery and scheduling are deliberately not claimed.

use core::arch::asm;

const IDT_ENTRIES: usize = 256;
const KERNEL_CODE_SELECTOR: u16 = 0x28;
const INTERRUPT_GATE: u8 = 0x8e;

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    attributes: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    const fn missing() -> Self {
        Self { offset_low: 0, selector: 0, ist: 0, attributes: 0, offset_mid: 0, offset_high: 0, reserved: 0 }
    }

    fn interrupt_gate(handler: u64) -> Self {
        Self {
            offset_low: handler as u16,
            selector: KERNEL_CODE_SELECTOR,
            ist: 0,
            attributes: INTERRUPT_GATE,
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            reserved: 0,
        }
    }
}

#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u64,
}

static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry::missing(); IDT_ENTRIES];

/// Install a kernel-owned IDT. The current stage intentionally leaves IF
/// disabled: hardware IRQs must not be enabled until PIC/APIC routing is ready.
pub fn initialize() {
    unsafe {
        let handler = exception_halt as usize as u64;
        let table = core::ptr::addr_of_mut!(IDT);
        let mut vector = 0usize;
        while vector < IDT_ENTRIES {
            core::ptr::write_volatile(table.cast::<IdtEntry>().add(vector), IdtEntry::interrupt_gate(handler));
            vector += 1;
        }
        let idtr = Idtr {
            limit: (core::mem::size_of::<[IdtEntry; IDT_ENTRIES]>() - 1) as u16,
            base: table as u64,
        };
        asm!("lidt [{}]", in(reg) &idtr, options(readonly, nostack, preserves_flags));
    }
}

extern "C" fn exception_halt() -> ! {
    crate::serial_write_str("\r\nNOVA OS: CPU exception/interrupt received; fail-stop handler engaged.\r\n");
    loop {
        unsafe { asm!("cli; hlt", options(nomem, nostack)); }
    }
}

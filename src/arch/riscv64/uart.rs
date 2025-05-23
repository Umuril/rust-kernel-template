use core::fmt::{Debug, Write};

use uart_16550;

pub(crate) struct Uart {
    pub(crate) imp: uart_16550::MmioSerialPort,
}

impl Uart {
    pub(crate) fn new() -> Self {
        let mut uart_16550 = unsafe { uart_16550::MmioSerialPort::new(0x10000000) };
        uart_16550.init();
        Self { imp: uart_16550 }
    }

    pub(crate) fn debug_value(&mut self, name: &str, val: u64) {
        let mut buffer = [0u8; 16];
        let s = u64_to_hex(val, &mut buffer);

        let _ = writeln!(self, "{name}: 0x{s}");
    }

    pub(crate) fn debug<T: Debug>(&mut self, name: &str, val: T) {
        let _ = writeln!(self, "{name}: {val:?}");
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.imp.write_str(s)
    }
}

fn u64_to_hex(mut num: u64, buf: &mut [u8; 16]) -> &str {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    // Always write all 16 hex digits
    for i in (0..16).rev() {
        let digit = (num & 0xF) as usize;
        buf[i] = HEX_CHARS[digit];
        num >>= 4;
    }

    // SAFETY: Only ASCII hex digits written
    unsafe { core::str::from_utf8_unchecked(&buf[..]) }
}

use core::{arch::naked_asm};

use embedded_io::{ErrorType, Write};
use uart_16550;

use crate::{kmain, Context};

#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe fn multiboot() -> ! {
    naked_asm!(
        "
        .long 0x1BADB002
        .long 0x00000000
        .long 0xE4524FFE
    "
    );
}

#[unsafe(no_mangle)]
pub unsafe fn kstart() -> ! {
    let mut uart_16550 = unsafe { uart_16550::SerialPort::new(0x3F8) };
    uart_16550.init();
    
    let mut uart = Uart { imp: uart_16550 };
    if uart.write_all(b"Bootstrapping kernel...\n").is_err() {
        unsafe { halt() };
    }

    let ctx = Context { primary_log: uart };

    kmain(ctx);
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe fn halt() -> ! {
    naked_asm!(
        "
        HLT
        JMP halt
    "
    );
}

struct Uart {
    imp: uart_16550::SerialPort,
}

impl ErrorType for Uart {
    type Error = embedded_io::ErrorKind;
}

impl Write for Uart {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut written = 0;

        for &c in buf {
            self.imp.send(c);
            written += 1;
        }

        Ok(written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

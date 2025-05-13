use arm_pl011_uart;
use core::ptr::NonNull;
use embedded_io::{self, Write};

pub(crate) struct Uart<'a> {
    imp: arm_pl011_uart::Uart<'a>,
}

impl<'a> Uart<'a> {
    pub(crate) fn new() -> Self {
        const UART_BASE_ADDR: *mut arm_pl011_uart::PL011Registers =
            0x0900_0000 as *mut arm_pl011_uart::PL011Registers;
        const BAUD_RATE: u32 = 115_200;
        const SYS_CLOCK: u32 = 24_000_000;

        let mmio = unsafe {
            arm_pl011_uart::UniqueMmioPointer::new(NonNull::new(UART_BASE_ADDR).unwrap())
        };
        let mut uart_pl011 = arm_pl011_uart::Uart::new(mmio);
        let config = arm_pl011_uart::LineConfig {
            data_bits: arm_pl011_uart::DataBits::Bits8,
            parity: arm_pl011_uart::Parity::None,
            stop_bits: arm_pl011_uart::StopBits::One,
        };
        let _ = uart_pl011.enable(config, BAUD_RATE, SYS_CLOCK);
        Self { imp: uart_pl011 }
    }
}

impl<'a> embedded_io::ErrorType for Uart<'a> {
    type Error = embedded_io::ErrorKind;
}

impl<'a> Write for Uart<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut written = 0;

        for &c in buf {
            self.imp.write_word(c);
            written += 1;
        }

        Ok(written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let _ = self.imp.flush();
        Ok(())
    }
}

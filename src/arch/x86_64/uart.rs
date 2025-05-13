use embedded_io;
use uart_16550;

pub(crate) struct Uart {
    imp: uart_16550::SerialPort,
}

impl Uart {
    pub(crate) fn new() -> Self {
        let mut uart_16550 = unsafe { uart_16550::SerialPort::new(0x3F8) };
        uart_16550.init();
        Self { imp: uart_16550 }
    }
}

impl embedded_io::ErrorType for Uart {
    type Error = embedded_io::ErrorKind;
}

impl embedded_io::Write for Uart {
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

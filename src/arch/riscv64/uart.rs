use uart_16550;

pub(crate) struct Uart {
    imp: uart_16550::MmioSerialPort,
}

impl Uart {
    pub(crate) fn new() -> Self {
        let mut uart_16550 = unsafe { uart_16550::MmioSerialPort::new(0x10000000) };
        uart_16550.init();
        Self { imp: uart_16550 }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.imp.write_str(s)
    }
}
